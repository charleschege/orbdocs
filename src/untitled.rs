use htmlstream::{
    tag_iter,
    HTMLTagState,
    HTMLTag,
};
//TODO disable the enter key or make it go to the next block
fn main() {
    let html_element = "<p>Content <i>now</i> <b>editable</b>too</p>";
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

    #[derive(Debug, Clone)]
    enum CursorState {
        Default,
        UnboundText,
        Open(OrbDocsTag),
        SelfClosing,
        BadTag,
    }

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
                        _ => OrbDocsTag::Image,
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

    dbg!(foo);

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
fn match_tag(tag: HTMLTag)  -> OrbDocsTag{
    if tag.state == HTMLTagState::Text {
        OrbDocsTag::UnboundText(tag.html)
    } else if tag.state == HTMLTagState::Opening {
        match tag.name.as_str() {
            "b" => OrbDocsTag::Bold(String::default()),
            "u" => OrbDocsTag::Underline(String::default()),
            "i" => OrbDocsTag::Italic(String::default()),
            _ => OrbDocsTag::BadTag,
        }
    }else {
        OrbDocsTag::BadTag
    }
}

struct OrbDocsElement {
    tag: OrbBlocks,
    content: Vec<OrbDocsTag>
}

#[derive(Debug, Clone)]
enum OrbDocsTag {
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
    BadTag,
}

enum OrbBlocks {
    TextBlocks
}
