fn main() {
    let spans = parse_md("*hello world* hello wo*rl*d _aaa_ ||s||asas***bold*** ***__biu__***");
    println!("{:#?}", spans);
}

#[derive(Debug, PartialEq, Eq)]
struct Span<'a> {
    kind: MarkdownKind,
    s: &'a str,
}

#[derive(Debug)]
struct Marker {
    kind: MarkdownKind,
    loc: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MarkdownKind {
    Italic,
    Bold,
    BoldItalic,
    BoldUnderline,
    BoldItalicUnderline,
    ItalicUnderline,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
}

impl MarkdownKind {
    const fn len(&self) -> usize {
        match self {
            MarkdownKind::Italic => 1,
            MarkdownKind::Bold => 2,
            MarkdownKind::Underline => 2,
            MarkdownKind::Strikethrough => 2,
            MarkdownKind::Spoiler => 2,
            MarkdownKind::Code => 1,
            MarkdownKind::BoldItalic => 3,
            MarkdownKind::BoldUnderline => 4,
            MarkdownKind::BoldItalicUnderline => 5,
            MarkdownKind::ItalicUnderline => 3,
        }
    }
}

enum State {
    Closing,
    Opening,
}

fn parse_md(input: &str) -> Vec<Span> {
    let mut out = Vec::new();
    let mut stack: Vec<Marker> = Vec::new();
    let char_indices: Vec<_> = input.char_indices().collect();

    let mut idx = 0;

    while idx < char_indices.len() {
        //eprintln!("another!");

        let (byte_idx, c) = char_indices[idx];

        //dbg!(&c);
        //

        let kind = match c {
            '*' => match char_indices.get(idx + 1) {
                Some((_, '*')) => match char_indices.get(idx + 2) {
                    Some((_, '_')) => match char_indices.get(idx + 3) {
                        Some((_, '_')) => MarkdownKind::BoldUnderline,
                        _ => MarkdownKind::BoldItalic,
                    },
                    Some((_, '*')) => match char_indices.get(idx + 3) {
                        Some((_, '_')) => match char_indices.get(idx + 3) {
                            Some((_, '_')) => MarkdownKind::BoldItalicUnderline,
                            _ => MarkdownKind::BoldItalic,
                        },
                        _ => MarkdownKind::BoldItalic,
                    },
                    _ => MarkdownKind::Bold,
                },
                _ => MarkdownKind::Italic,
            },
            '_' => match char_indices.get(idx + 1) {
                Some((_, '*')) => match char_indices.get(idx + 2) {
                    Some((_, '*')) => MarkdownKind::BoldItalic,
                    _ => MarkdownKind::Italic,
                },
                Some((_, '_')) => match char_indices.get(idx + 2) {
                    Some((_, '*')) => match char_indices.get(idx + 3) {
                        Some((_, '*')) => match char_indices.get(idx + 4) {
                            Some((_, '*')) => MarkdownKind::BoldItalicUnderline,
                            _ => MarkdownKind::BoldUnderline,
                        },
                        _ => MarkdownKind::ItalicUnderline,
                    },
                    Some((_, '_')) => MarkdownKind::ItalicUnderline,
                    _ => MarkdownKind::Underline,
                },
                _ => MarkdownKind::Italic,
            },
            '~' => match char_indices.get(idx + 1) {
                Some((_, '~')) => MarkdownKind::Strikethrough,
                _ => {
                    idx += 1;
                    continue;
                }
            },
            '|' => match char_indices.get(idx + 1) {
                Some((_, '|')) => MarkdownKind::Spoiler,
                _ => {
                    idx += 1;
                    continue;
                }
            },
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
                let s = &input[loc + kind.len()..byte_idx];
                out.push(Span { kind, s })
            }
        }

        idx += kind.len();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold() {
        let spans = parse_md("**hello world**");
        println!("{:#?}", spans);
        assert_eq!(
            spans[0],
            Span {
                kind: MarkdownKind::Bold,
                s: "hello world"
            }
        );
    }
    #[test]
    fn italics1() {
        let spans = parse_md("*hello world*");
        println!("{:#?}", spans);
        assert_eq!(
            spans[0],
            Span {
                kind: MarkdownKind::Italic,
                s: "hello world"
            }
        );
    }
    #[test]
    fn italics2() {
        let spans = parse_md("_hello world_");
        println!("{:#?}", spans);
        assert_eq!(
            spans[0],
            Span {
                kind: MarkdownKind::Italic,
                s: "hello world"
            }
        );
    }
    #[test]
    fn italics2bold() {
        let spans = parse_md("_**hello world**_");
        println!("{:#?}", spans);
        assert_eq!(
            spans[0],
            Span {
                kind: MarkdownKind::BoldItalic,
                s: "hello world"
            }
        );
    }
    #[test]
    fn bolditalics1() {
        let spans = parse_md("***hello world***");
        println!("{:#?}", spans);
        assert_eq!(
            spans[0],
            Span {
                kind: MarkdownKind::BoldItalic,
                s: "hello world"
            }
        );
    }
    #[test]
    fn bolditalics2() {
        let spans = parse_md("**_hello world_**");
        println!("{:#?}", spans);
        assert_eq!(
            spans[0],
            Span {
                kind: MarkdownKind::BoldItalic,
                s: "hello world"
            }
        );
    }
}
