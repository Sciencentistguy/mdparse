use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
/// A &str alongside its markdown type and its location within the input.
pub struct Span<'a> {
    pub kind: Option<MarkdownKind>,
    pub s: &'a str,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub(crate) struct Marker {
    pub(crate) kind: Option<MarkdownKind>,
    pub(crate) loc: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// All the types of markdown formatting that a string can have in Discord markdown
pub enum MarkdownKind {
    Italics,
    Bold,
    BoldItalics,
    BoldUnderscore,
    BoldItalicsUnderscore,
    ItalicsUnderscore,
    Underscore,
    Strikethrough,
    Spoiler,
    InlineCode,
}

impl MarkdownKind {
    pub(crate) const fn len(self) -> usize {
        match self {
            MarkdownKind::Italics => 1,
            MarkdownKind::Bold => 2,
            MarkdownKind::Underscore => 2,
            MarkdownKind::Strikethrough => 2,
            MarkdownKind::Spoiler => 2,
            MarkdownKind::InlineCode => 1,
            MarkdownKind::BoldItalics => 3,
            MarkdownKind::BoldUnderscore => 4,
            MarkdownKind::BoldItalicsUnderscore => 5,
            MarkdownKind::ItalicsUnderscore => 3,
        }
    }
}

pub(crate) enum State {
    Closing,
    Opening,
    OpeningFromNone,
    ClosingToNone
}

#[derive(Debug)]
pub enum MdparseError {
    OutOfRangeError(usize),
    InternalError(String),
}
