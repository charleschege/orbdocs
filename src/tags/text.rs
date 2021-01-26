use web_sys::HtmlDocument;
use yew::{html, html::Html};

pub enum TextOps {
    Bold(bool),
    Underline(bool),
    StrikeThrough(bool),
    Italic(bool),
    SuperScript(bool),
    SubScript(bool),
    LineHeight,
    FontSize(FontSizeOps),
    FontWeight(FontWeightOps),
    ForeColor,
    BackColor,
}

pub enum FontWeightOps {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Normal,
}
