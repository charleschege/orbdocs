//use js_sys::{Function, Object, Reflect, WebAssembly};
use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, 
    Element, 
    HtmlElement, 
    MutationObserverInit, 
    MutationObserver,
    HtmlCollection
};

mod parser;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    
    body.append_child(&toolbar(&document))?;
    body.append_child(&editor(&document))?;

    set_editable(&document);
    //bold_node(&document);
    //italic_node(&document);
    //underline_node(&document);
    //strikethrough_node(&document);
    observer(&document).unwrap();

    Ok(())
}

//let a = Closure::wrap(Box::new(move || update_time(&current_time)) as Box<dyn Fn()>);
//let js_function = a.as_ref().unchecked_ref();

pub fn editor(document: &Document) -> Element {
    let editor_section = document.create_element("section").unwrap();
    let editor_main_div = document.create_element("div").unwrap();
    editor_main_div.set_id("editor");
    editor_main_div.set_class_name("frow row-around");

    editor_section.append_child(&editor_main_div).unwrap();

    editor_section
}

#[wasm_bindgen(catch)]
pub fn observer(document: &Document) -> anyhow::Result<(), JsValue> {
    let element = document.get_element_by_id("editor").unwrap();
    let observe_element = document.get_element_by_id("editor").unwrap();
    let foo_call = Closure::wrap(
        Box::new(move || {            
            /*unsafe {web_sys::console::log_1(
                &observe_element
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .outer_html()
                    .into()
            )}
            
            unsafe {web_sys::console::log_1(
                &observe_element
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .inner_text()
                    .into()
            )}
            
            unsafe {web_sys::console::log_1(
                //When getting the inner nodes from a child node
                &collection
                .item(0)
                .unwrap()
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .children()
            )}
            */

            /*unsafe {web_sys::console::log_1(
                collection
                    .item(0)
                    .unwrap()
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
            )}*/

            let collection = &observe_element
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .children()
                    .item(0)
                    .unwrap()
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .outer_html();
            
            unsafe {web_sys::console::log_1(
                &format!("{:?}", parser::data(&collection)).into()
            )}
        }) as Box<dyn FnMut()>);
    
    let observer = MutationObserver::new(&foo_call.as_ref().unchecked_ref())?;
    observer.observe_with_options(&element, 
        MutationObserverInit::new()
            .character_data_old_value(true)
            .character_data(true)
            .subtree(true)
    ).unwrap();
    
    foo_call.forget();

    Ok(())
}

pub fn set_editable(document: &Document) {
    let node = document.get_element_by_id("editor").unwrap();
    //TODO set doc height and background and width
    //TODO Add a node
    node.set_inner_html("<p>Content <del>now</del> <b>editable</b><svg>foooobarrr</svg></p>");
    node
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_content_editable("true");
}

pub fn toolbar(document: &Document) -> Element {
    let val = document.create_element("div").unwrap();
    val.set_id("toolbar");
    val.set_class_name("frow row-around");

    val
}

pub fn bold_node(document: &Document) -> Element{
    let foo = document.create_element("button").unwrap();
    foo.set_class_name("fa fa-bold");
    
    let node = document.get_element_by_id("toolbar").unwrap();
    let onclick = Closure::wrap(
        Box::new(move || {unsafe {web_sys::console::log_1(&"bold".into())}
        }) as Box<dyn FnMut()>);

    node
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(onclick.as_ref().unchecked_ref()));

    onclick.forget(); //FIXME check why this causes memory leaks in wasm-bindgen

    node.append_child(&foo).unwrap();
    

    node
}

pub fn italic_node(document: &Document) -> Element{
    let foo = document.create_element("button").unwrap();
    foo.set_class_name("fa fa-italic");
    
    let node = document.get_element_by_id("toolbar").unwrap();
    node.append_child(&foo).unwrap();

    node
}

pub fn underline_node(document: &Document) -> Element{
    let foo = document.create_element("button").unwrap();
    foo.set_class_name("fa fa-underline");
    
    let node = document.get_element_by_id("toolbar").unwrap();
    node.append_child(&foo).unwrap();

    node
}

pub fn strikethrough_node(document: &Document) -> Element{
    let foo = document.create_element("button").unwrap();
    foo.set_class_name("fa fa-strikethrough");
    
    let node = document.get_element_by_id("toolbar").unwrap();
    node.append_child(&foo).unwrap();

    node
}

/*

// And now that our demo is ready to go let's switch things up so
    // everything is displayed and our loading prompt is hidden.
    document
        .get_element_by_id("loading")
        .expect("should have #loading on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;
    document
        .get_element_by_id("script")
        .expect("should have #script on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#script should be an `HtmlElement`")
        .style()
        .set_property("display", "block")?;

    document
        .get_element_by_id("green-square")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));

*/