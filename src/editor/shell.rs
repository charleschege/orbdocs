use yew::prelude::*;

pub fn editor_shell() -> Html {
    html! {
        <div class="frow direction-column">
            <div id="editor" contenteditable=true>
                <h1>{"Simple Html editor"}</h1>
                <p>{"Good to start"}</p>
            </div>
        </div>
    }
}