use crate::minecraft;

#[derive(Debug)]
pub enum Token {
    Text(Box<str>),
    /// A hidden node to control text formatting.
    Format(minecraft::Format),
    Space,
    LineBreak,
    ParagraphBreak,
    /// A page break.
    ThematicBreak,
}

impl From<&mut Vec<char>> for Token {
    /// Drain a `Vec<char>` to build a text node.
    fn from(value: &mut Vec<char>) -> Self {
        Self::Text(value.drain(..).collect::<Box<str>>())
    }
}
