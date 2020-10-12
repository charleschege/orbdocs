use yew::{html, html::Html};
use web_sys::HtmlDocument;

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

pub fn textops_block() -> Html{

    let window = web_sys::window().unwrap();
    let mydocument = window.document().unwrap();

    html! {
        <div class="frow">
            <button class="tool-items fa fa-bold" onlick=""></button>
            <button class="tool-items fa fa-underline"></button>    
            <button class="tool-items fa fa-italic"></button>
            <button class="tool-items fa fa-strikethrough"></button>

            <button class="tool-items fa fa-superscript"></button>
            <button class="tool-items fa fa-subscript"></button>
            <button class="tool-items fa fa-text-height"></button>
            <button class="tool-items fa fa-text-width"></button>

            { FontSizeOps::to_html() }
            { FontWeightOps::to_html() }

            <hr/>
            <ul id="foo">
                <li>{"Polar"}</li>
            </ul>
            
        </div>
    }
}

pub enum FontSizeOps {
    Five,
    Ten,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
    OneHundred,
    OneHundredTen,
    OneHundredTwenty,
    OneHundredThirty,
    OneHundredForty,
    OneHundredFifty,
    NotApplicable,
}

impl FontSizeOps {
    pub fn to_html() -> Html {
        html! {
            <select name="fontsize-ops" id="">
                <option value="Five">{"5%"}</option>
                <option value="Ten">{"10%"}</option>
                <option value="Twenty">{"20%"}</option>
                <option value="Thirty">{"30%"}</option>
                <option value="Forty">{"40%"}</option>
                <option value="Fifty">{"50%"}</option>
                <option value="Sixty">{"60%"}</option>
                <option value="Seventy">{"70%"}</option>
                <option value="Eighty">{"80%"}</option>
                <option value="Ninety">{"90%"}</option>
                <option value="OneHundred">{"100%"}</option>
                <option value="OneHundredTen">{"110%"}</option>
                <option value="OneHundredTwenty">{"120%"}</option>
                <option value="OneHundredThirty">{"130%"}</option>
                <option value="OneHundredForty">{"140%"}</option>
                <option value="OneHundredFifty">{"150%"}</option>
            </select>
        }
    }

    pub fn from_html(value: &str) -> Self {
        match value {
            "Five" => Self::Five,
            "Ten" => Self::Ten,
            "Twenty" => Self::Twenty,
            "Thirty" => Self::Thirty,
            "Forty" => Self::Forty,
            "Fifty" => Self::Fifty,
            "Sixty" => Self::Sixty,
            "Seventy" => Self::Seventy,
            "Eighty" => Self::Eighty,
            "Ninety" => Self::Ninety,
            "OneHundred" => Self::OneHundred,
            "OneHundredTen" => Self::OneHundredTen,
            "OneHundredTwenty" => Self::OneHundredTwenty,
            "OneHundredThirty" => Self::OneHundredThirty,
            "OneHundredForty" => Self::OneHundredForty,
            "OneHundredFifty" => Self::OneHundredFifty,
            _ => Self::NotApplicable,
        }
    }
}

pub enum FontWeightOps {
    UltraLight,
    Light,
    Medium,
    Heavy,
    SuperHeavy,
    NotApplicable,
}

impl FontWeightOps {
    pub fn to_html() -> Html {
        html! {
            <select name="fontweight-ops" id="">
                <option value="UltraLight">{"UltraLight"}</option>
                <option value="Light">{"Light"}</option>
                <option value="Medium">{"Medium"}</option>
                <option value="Heavy">{"Heavy"}</option>
                <option value="SuperHeavy">{"SuperHeavy"}</option>
            </select>
        }
    }

    pub fn from_html(value: &str) -> Self {
        match value {
            "UltraLight" => Self::UltraLight,
            "Light" => Self::Light,
            "Medium" => Self::Medium,
            "Heavy" => Self::Heavy,
            "SuperHeavy" => Self::SuperHeavy,
            _ => Self::NotApplicable,
        }
    }
}