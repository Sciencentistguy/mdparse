use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
/// A &str alongside its markdown type and its location within the input.
pub struct Span<'a> {
    pub kind: MarkdownKind,
    pub s: &'a str,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub(crate) struct Marker {
    pub(crate) kind: MarkdownKind,
    pub(crate) loc: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// All the types of markdown formatting that a string can have in Discord markdown
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
    pub(crate) const fn len(self) -> usize {
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

pub(crate) enum State {
    Closing,
    Opening,
}

#[derive(Debug)]
pub enum MdparseError {
    OutOfRangeError(usize),
    InternalError(String),
}
