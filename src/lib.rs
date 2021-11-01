#[cfg(test)]
mod test;

mod types;

use types::*;

// FIXME: the string `**_a**_` parses as bold italics, but discord renders it as bold with
// underscores
pub fn parse_md(input: &str) -> Vec<Span> {
    let mut out = Vec::new();
    let mut stack: Vec<Marker> = Vec::new();
    let char_indices: Vec<_> = input.char_indices().collect();

    let mut idx = 0;

    while idx < char_indices.len() {
        let (byte_idx, c) = char_indices[idx];
        let rem = &input[byte_idx..];

        let kind = match c {
            '*' if rem.starts_with("***__") || rem.starts_with("**___") => {
                MarkdownKind::BoldItalicUnderline
            }
            '*' if rem.starts_with("**__") => MarkdownKind::BoldUnderline,
            '*' if rem.starts_with("**_") || rem.starts_with("***") => MarkdownKind::BoldItalic,
            '*' if rem.starts_with("**") => MarkdownKind::Bold,
            '*' => MarkdownKind::Italic,
            '_' if rem.starts_with("__***") || rem.starts_with("__**_") => {
                MarkdownKind::BoldItalicUnderline
            }
            '_' if rem.starts_with("_**") => MarkdownKind::BoldItalic,
            '_' if rem.starts_with("__*") || rem.starts_with("___") => {
                MarkdownKind::ItalicUnderline
            }
            '_' if rem.starts_with("__") => MarkdownKind::Underline,
            '_' => MarkdownKind::Italic,
            '~' if rem.starts_with("~~") => MarkdownKind::Strikethrough,
            '|' if rem.starts_with("||") => MarkdownKind::Spoiler,
            '`' => MarkdownKind::Code,
            _ => {
                idx += 1;
                continue;
            }
        };

        // either:
        //  stack is empty, this is the beginning, open a span
        //  stack is not empty, top.kind is other, open a new span
        //  stack is not empty, top.kind is the same as kind, close the top span
        //
        dbg!(&stack.last());

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
                let Marker { kind, loc } = stack.pop().expect("Stack was empty and closing");
                let range = loc + kind.len()..byte_idx;
                let s = &input[range.clone()];
                out.push(Span { kind, s, range })
            }
        }

        idx += kind.len();
    }
    out
}
