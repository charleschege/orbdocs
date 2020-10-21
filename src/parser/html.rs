use htmlstream::{
    tag_iter,
    HTMLTagState,
    HTMLTag,
};

//let html_element = "<p>Content <i>now</i> <b>editable</b>too</p>";

//TODO disable the enter key or make it go to the next block
pub fn data(html_element: &str) -> Vec<OrbDocsTag> {
    let mut container = Vec::new();

    let check_tag = tag_iter(html_element).take(1).next();

    match check_tag {
        Some((_, tag)) => match tag.name.as_str() {
            "p" => {
                match_tag(tag);
                for (_, tag) in tag_iter(html_element) {
                    if tag.state != HTMLTagState::Closing {
                        container.push(tag);
                    }
                }
            },
            _ => { dbg!("None"); },
        },
        None => { dbg!("Error! Tag doesnt exist"); },
    }

    let mut foo = Vec::new();

    let mut cursor_state = CursorState::Default;


    // Remove the first index as it is an element
    // The fist element will also cause `BadTag` error
    container.remove(0); 

    for tag in container {
        match &cursor_state {
            CursorState::Default => {
                match tag.state {
                    HTMLTagState::Text => {
                        foo.push(OrbDocsTag::UnboundText(tag.html));
                        cursor_state = CursorState::UnboundText;
                    },
                    HTMLTagState::Opening => {
                        cursor_state = CursorState::Open(match_tag(tag));
                    },
                    HTMLTagState::SelfClosing => {
                        cursor_state = CursorState::SelfClosing;
                    },
                    HTMLTagState::Closing => {
                        cursor_state = CursorState::BadTag;
                    },
                }
            },
            CursorState::UnboundText => {
                if tag.state == HTMLTagState::Text {
                    foo.push(OrbDocsTag::UnboundText(tag.html));
                    cursor_state = CursorState::UnboundText;
                }else if tag.state == HTMLTagState::Opening {
                    cursor_state = CursorState::Open(match_tag(tag));
                }else if tag.state == HTMLTagState::SelfClosing {
                    cursor_state = CursorState::SelfClosing;
                }
            },
            CursorState::SelfClosing => {
                if tag.state == HTMLTagState::Text {
                    foo.push(OrbDocsTag::UnboundText(tag.html));
                    cursor_state = CursorState::UnboundText;
                }else if tag.state == HTMLTagState::Opening {
                    cursor_state = CursorState::Open(match_tag(tag));
                }else if tag.state == HTMLTagState::SelfClosing {
                    cursor_state = CursorState::SelfClosing;
                }
            },
            CursorState::Open(inner) => {
                if tag.state == HTMLTagState::Text {
                    let data = match inner {
                        OrbDocsTag::UnboundText(_) => OrbDocsTag::UnboundText(tag.html),
                        OrbDocsTag::Bold(_) => OrbDocsTag::Bold(tag.html),
                        OrbDocsTag::Italic(_) => OrbDocsTag::Italic(tag.html),
                        OrbDocsTag::Underline(_) => OrbDocsTag::Underline(tag.html),
                        OrbDocsTag::DeletedText(_) => OrbDocsTag::DeletedText(tag.html),
                        OrbDocsTag::SuperScript(_) => OrbDocsTag::SuperScript(tag.html),
                        OrbDocsTag::SubScript(_) => OrbDocsTag::SubScript(tag.html),
                        OrbDocsTag::Heading(_) => OrbDocsTag::Heading(tag.html),
                        OrbDocsTag::Justify(_) => OrbDocsTag::Justify(tag.html),
                        OrbDocsTag::Indent(_) => OrbDocsTag::Indent(tag.html),
                        OrbDocsTag::Svg(_) => OrbDocsTag::Svg(tag.html),
                        _ => OrbDocsTag::BadTag,
                    };
                    foo.push(data);
                    cursor_state = CursorState::Default;
                }else if tag.state == HTMLTagState::Opening {
                    cursor_state = CursorState::BadTag;
                }else if tag.state == HTMLTagState::SelfClosing {
                    cursor_state = CursorState::BadTag;
                }
            },
            CursorState::BadTag => {
                foo.push(OrbDocsTag::BadTag);
            },
        }
    }

    foo

}
// Create a cursor
//Check if opening tag
//If opening tag check index
//Take next index
//Skip next index
//Place cursor past here
//Else if self closing tag
//take next index
//Current cursor pos in next index
pub fn match_tag(tag: HTMLTag)  -> OrbDocsTag {
    if tag.state == HTMLTagState::Text {
        OrbDocsTag::UnboundText(tag.html)
    } else if tag.state == HTMLTagState::Opening {
        match tag.name.as_str() {
            "b" => OrbDocsTag::Bold(String::default()),
            "u" => OrbDocsTag::Underline(String::default()),
            "i" => OrbDocsTag::Italic(String::default()),
            "del" => OrbDocsTag::DeletedText(String::default()),
            "svg" => OrbDocsTag::Svg(String::default()),
            "sup" => OrbDocsTag::SuperScript(String::default()),
            "sub" => OrbDocsTag::SubScript(String::default()),
            "h1" => OrbDocsTag::Heading(String::default()),
            "h2" => OrbDocsTag::Heading(String::default()),
            "h3" => OrbDocsTag::Heading(String::default()),
            "h4" => OrbDocsTag::Heading(String::default()),
            "h5" => OrbDocsTag::Heading(String::default()),
            "h6" => OrbDocsTag::Heading(String::default()),
            "justify" => OrbDocsTag::Heading(String::default()), //FIXME
            "indent" => OrbDocsTag::Indent(String::default()), //FIXME
            _ => OrbDocsTag::BadTag,
        }
    }else {
        OrbDocsTag::BadTag
    }
}

#[derive(Debug, Clone)]
pub enum CursorState {
    Default,
    UnboundText,
    Open(OrbDocsTag),
    SelfClosing,
    BadTag,
}


pub struct OrbDocsElement {
    tag: OrbBlocks,
    content: Vec<OrbDocsTag>
}

#[derive(Debug, Clone)]
pub enum OrbDocsTag {
    Paragraph,
    UnboundText(String),
    OrderedList,
    UnorderedList,
    Code,
    Link,
    Image,
    Bold(String),
    Italic(String),
    Underline(String),
    DeletedText(String),
    SuperScript(String),
    SubScript(String),
    Heading(String),
    Justify(String),
    Indent(String),
    Svg(String),
    BadTag,
}

pub enum OrbBlocks {
    TextBlocks
}
