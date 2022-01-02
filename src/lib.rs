#[cfg(test)]
mod test;

mod types;

use types::*;

pub type Result<T> = core::result::Result<T, MdparseError>;

fn get_token_kind(c: char, rem: &str) -> Option<MarkdownKind> {
    match c {
        '*' if rem.starts_with("***__") || rem.starts_with("**___") => {
            Some(MarkdownKind::BoldItalicsUnderscore)
        }
        '*' if rem.starts_with("**__") => Some(MarkdownKind::BoldUnderscore),
        '*' if rem.starts_with("**_") || rem.starts_with("***") => Some(MarkdownKind::BoldItalics),
        '*' if rem.starts_with("**") => Some(MarkdownKind::Bold),
        '*' => Some(MarkdownKind::Italics),
        '_' if rem.starts_with("__***")
            || rem.starts_with("__**_")
            || rem.starts_with("___**")
            || rem.starts_with("_**__") =>
        {
            Some(MarkdownKind::BoldItalicsUnderscore)
        }
        '_' if rem.starts_with("__**") => Some(MarkdownKind::BoldUnderscore),
        '_' if rem.starts_with("_**") => Some(MarkdownKind::BoldItalics),
        '_' if rem.starts_with("__*") || rem.starts_with("___") => {
            Some(MarkdownKind::ItalicsUnderscore)
        }
        '_' if rem.starts_with("__") => Some(MarkdownKind::Underscore),
        '_' => Some(MarkdownKind::Italics),
        '~' if rem.starts_with("~~") => Some(MarkdownKind::Strikethrough),
        '|' if rem.starts_with("||") => Some(MarkdownKind::Spoiler),
        '`' => Some(MarkdownKind::InlineCode),
        _ => None,
    }
}

// FIXME: the string `**_a**_` parses as bold italics, but discord renders it as bold with
// underscores

// init stack with null tag
// Read char
// if char is control char, open tag
// if not, continue looking for control char
// if control char, then close null tag, open control tag
// if currently in tag of that kind, close control tag, possibly open null tag
// repeat

/// Parse a string as Discord markdown.
pub fn parse(input: &str) -> Result<Vec<Span>> {
    let mut out = Vec::new();
    let mut stack: Vec<Marker> = vec![Marker { kind: None, loc: 0 }];
    let mut char_indices = input.char_indices();

    while let Some((byte_idx, c)) = char_indices.next() {
        let rem = input
            .get(byte_idx..)
            .ok_or(MdparseError::OutOfRangeError(byte_idx))?;

        let current_token_kind = match get_token_kind(c, rem) {
            Some(x) => x,
            None => {
                continue;
            }
        };

        // either:
        //  stack is empty, this is the beginning, open a span
        //  stack is not empty, top.kind is other, open a new span
        //  stack is not empty, top.kind is the same as kind, close the top span

        // stack is just null tag
        // stack is tag that does not match
        // stack is tag that does match
        let state = match stack.last() {
            None => unreachable!("there should always be a None tag"),
            Some(Marker { kind: None, loc: _ }) => State::OpeningFromNone,
            Some(Marker {
                kind: Some(top_kind),
                loc: _,
            }) if *top_kind != current_token_kind => State::Opening,
            Some(_) if stack.len() == 1 => State::ClosingToNone,
            Some(_) => State::Closing,
        };

        match state {
            State::OpeningFromNone => {
                let Marker { kind, loc } = stack
                    .pop()
                    .ok_or_else(|| MdparseError::InternalError("Stack was empty".into()))?;

                debug_assert!(kind.is_none());

                let range = loc..byte_idx;
                if !range.is_empty() {
                    let s = input
                        .get(range.clone())
                        .ok_or(MdparseError::OutOfRangeError(loc))?;
                    out.push(Span { kind, s, range });
                }

                stack.push(Marker {
                    kind: Some(current_token_kind),
                    loc: byte_idx,
                });
            }
            State::Opening => stack.push(Marker {
                kind: Some(current_token_kind),
                loc: byte_idx,
            }),
            State::Closing => {
                let Marker { kind, loc } = stack
                    .pop()
                    .ok_or_else(|| MdparseError::InternalError("Stack was empty".to_owned()))?;
                let kind = kind.ok_or_else(|| {
                    MdparseError::InternalError(
                        "Stack was None and the parser was in Closing".to_owned(),
                    )
                })?;
                let range = loc + kind.len()..byte_idx;
                let s = input
                    .get(range.clone())
                    .ok_or_else(|| MdparseError::OutOfRangeError(loc + kind.len()))?;
                out.push(Span {
                    kind: Some(kind),
                    s,
                    range,
                });
                debug_assert!(!stack.is_empty());
            }
            State::ClosingToNone => {
                let Marker { kind, loc } = stack
                    .pop()
                    .ok_or_else(|| MdparseError::InternalError("Stack was empty".to_owned()))?;
                let kind = kind.ok_or_else(|| {
                    MdparseError::InternalError(
                        "Stack was None and the parser was in Closing".to_owned(),
                    )
                })?;
                let range = loc + kind.len()..byte_idx;
                let s = input
                    .get(range.clone())
                    .ok_or_else(|| MdparseError::OutOfRangeError(loc + kind.len()))?;
                out.push(Span {
                    kind: Some(kind),
                    s,
                    range,
                });
                let end_of_token = byte_idx + kind.len();
                stack.push(Marker {
                    kind: None,
                    loc: end_of_token,
                });
                debug_assert_eq!(stack.len(), 1);
            }
        }

        for _ in 0..current_token_kind.len() {
            let _ = char_indices.next();
        }
    }

    if let [Marker { kind: None, loc }] = stack.as_slice() {
        let loc = *loc;
        let range = loc..input.len();
        if !range.is_empty() {
            let s = input
                .get(range.clone())
                .ok_or(MdparseError::OutOfRangeError(loc))?;
            out.push(Span {
                kind: None,
                s,
                range,
            });
        }
    }

    if out.is_empty() {
        out.push(Span {
            kind: None,
            s: input,
            range: 0..input.len(),
        });
    }

    Ok(out)
}
