use log::trace;
use mogwai::prelude::*;
use web_sys::HtmlDocument;

pub enum TextOps {
    Bold,
    Underline,
    StrikeThrough,
    Italic,
    SuperScript,
    SubScript,
}

impl TextOps {
    pub fn tooltip() -> ViewBuilder<HtmlElement> {
        let (tx_tooltip, rx_tooltip) = txrx();

        builder! {
            <div class="frow full-width row-center p-10">
                <button class="tooltip m-5 p-5" on:click=tx_tooltip.contra_map(|ev_tooltip:&Event|{
                    trace!("Tooltip Ev: {:?}", ev_tooltip);
                    { TextOps::bold_ev(); }
                })>"B"</button>
                <span class="tooltip m-5 p-5">"I"</span>
                <span class="tooltip m-5 p-5">"U"</span>
                <span class="tooltip m-5 p-5">"S"</span>
                <span class="tooltip m-5 p-5">"xT"</span>
                <span class="tooltip m-5 p-5">"Tx"</span>
            </div>
        }
    }

    fn bold_ev() {
        let document = mogwai::utils::document();
        let bold = document.create_element("b").unwrap();

        trace!(
            "Selection changed: {:?}",
            match document.get_selection().unwrap() {
                Some(selection) => {
                    let range = selection.get_range_at(0).unwrap();
                    let command_state =
                        TextOps::command_state(document.dyn_ref::<HtmlDocument>().unwrap())
                            .unwrap();

                    if command_state.bold {
                        trace!("bold_text_exists");
                    } else {
                        range.surround_contents(&bold).unwrap();
                        trace!("text_bolded");
                    }
                    enquote::unquote(&format!("{:?}", range.to_string())).unwrap()
                }
                None => "NONE".to_string(),
            }
        );
    }

    fn command_state(html_document: &HtmlDocument) -> Result<CommandState, JsValue> {
        Ok(CommandState {
            bold: html_document.query_command_state("bold")?,
            italic: html_document.query_command_state("italic")?,
            underline: html_document.query_command_state("underline")?,
            del: html_document.query_command_state("strikethrough")?,
            subscript: html_document.query_command_state("subscript")?,
            superscript: html_document.query_command_state("superscript")?,
        })
    }
}

#[derive(Debug)]
struct CommandState {
    bold: bool,
    italic: bool,
    underline: bool,
    del: bool,
    subscript: bool,
    superscript: bool,
}
#[derive(Debug)]
struct CommandValue {
    //TODO Document.queryCommandValue
}

struct Text {
    content: String,
    state: CommandState,
}

pub enum Heading {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}
