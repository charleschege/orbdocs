#![allow(unused_braces)]
use log::{trace, Level};
use mogwai::prelude::*;
use mogwai::utils::document;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod editor;
use editor::*;

mod blocks;
use blocks::TextOps;

#[wasm_bindgen]
pub fn main(parent_id: Option<String>) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let (tx, rx) = txrx();

    let view = View::from(builder! {
        <div data-editor-container>
            { TextOps::tooltip() }
            <p contenteditable="true" document:selectionchange=tx.contra_map(|_|
                ()
            )>
                "Hello x43·spacemachine"
            </p>
        </div>
    });

    if let Some(id) = parent_id {
        let parent = utils::document().get_element_by_id(&id).unwrap();

        view.run_in_container(&parent)
    } else {
        view.run()
    }
}

//TODO check if selecting textops multiple times creates nested same tags and try to flatten
// TODO make tracing optional
