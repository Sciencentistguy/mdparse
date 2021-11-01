#[cfg(test)]
mod test;

mod types;

use types::*;

pub type Result<T> = core::result::Result<T, MdparseError>;

fn get_token_kind(c: char, rem: &str) -> Option<MarkdownKind> {
    match c {
        '*' if rem.starts_with("***__") || rem.starts_with("**___") => {
            Some(MarkdownKind::BoldItalicUnderline)
        }
        '*' if rem.starts_with("**__") => Some(MarkdownKind::BoldUnderline),
        '*' if rem.starts_with("**_") || rem.starts_with("***") => Some(MarkdownKind::BoldItalic),
        '*' if rem.starts_with("**") => Some(MarkdownKind::Bold),
        '*' => Some(MarkdownKind::Italic),
        '_' if rem.starts_with("__***") || rem.starts_with("__**_") => {
            Some(MarkdownKind::BoldItalicUnderline)
        }
        '_' if rem.starts_with("_**") => Some(MarkdownKind::BoldItalic),
        '_' if rem.starts_with("__*") || rem.starts_with("___") => {
            Some(MarkdownKind::ItalicUnderline)
        }
        '_' if rem.starts_with("__") => Some(MarkdownKind::Underline),
        '_' => Some(MarkdownKind::Italic),
        '~' if rem.starts_with("~~") => Some(MarkdownKind::Strikethrough),
        '|' if rem.starts_with("||") => Some(MarkdownKind::Spoiler),
        '`' => Some(MarkdownKind::Code),
        _ => None,
    }
}

// FIXME: the string `**_a**_` parses as bold italics, but discord renders it as bold with
// underscores
/// Parse a string as Discord markdown.
pub fn parse_md(input: &str) -> Result<Vec<Span>> {
    let mut out = Vec::new();
    let mut stack: Vec<Marker> = Vec::new();
    let mut char_indices = input.char_indices();

    while let Some((byte_idx, c)) = char_indices.next() {
        let rem = input
            .get(byte_idx..)
            .ok_or(MdparseError::OutOfRangeError(byte_idx))?;

        let kind = match get_token_kind(c, rem) {
            Some(x) => x,
            None => {
                continue;
            }
        };

        // either:
        //  stack is empty, this is the beginning, open a span
        //  stack is not empty, top.kind is other, open a new span
        //  stack is not empty, top.kind is the same as kind, close the top span
        //
        let state = match stack.last() {
            None => State::Opening,
            Some(top) if top.kind != kind => State::Opening,
            Some(_) => State::Closing,
        };

        match state {
            State::Opening => stack.push(Marker {
                kind,
                loc: byte_idx,
            }),
            State::Closing => {
                let Marker { kind, loc } = stack.pop().ok_or_else(|| {
                    MdparseError::InternalError(
                        "Stack was empty and the parser was in Closing".to_owned(),
                    )
                })?;
                let range = loc + kind.len()..byte_idx;
                let s = input
                    .get(range.clone())
                    .ok_or_else(|| MdparseError::OutOfRangeError(loc + kind.len()))?;
                out.push(Span { kind, s, range });
            }
        }

        for _ in 0..kind.len() {
            let _ = char_indices.next();
        }
    }
    Ok(out)
}
