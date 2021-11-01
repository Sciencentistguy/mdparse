use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Span<'a> {
    pub kind: MarkdownKind,
    pub s: &'a str,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct Marker {
    pub kind: MarkdownKind,
    pub loc: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MarkdownKind {
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
    pub const fn len(&self) -> usize {
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

pub enum State {
    Closing,
    Opening,
}
