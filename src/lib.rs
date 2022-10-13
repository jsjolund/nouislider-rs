use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub min: i64,
    pub max: i64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Options {
    pub start: Vec<i64>,
    pub connect: bool,
    pub range: Range,
    pub tooltips: bool,
}

pub type JsVec = Vec<JsValue>;
// pub type CallbackValues = (JsVec, JsValue, JsVec, JsValue, JsVec, JsValue);
// pub type Callback = Closure<dyn Fn(CallbackValues)>;
pub type Callback = Closure<dyn Fn(JsVec, JsValue, JsVec, JsValue, JsVec, JsValue)>;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(js_name = create)]
    pub type NoUiSlider;

    #[wasm_bindgen(constructor, js_class = create, js_namespace = noUiSlider)]
    pub fn new_with_options(target: &HtmlElement, options: &JsValue) -> NoUiSlider;

    #[wasm_bindgen(method)]
    pub fn get(this: &NoUiSlider) -> Vec<JsValue>;

    #[wasm_bindgen(method, js_name = get)]
    pub fn get_with_options(this: &NoUiSlider, formatting: bool) -> Vec<JsValue>;

    #[wasm_bindgen(method)]
    pub fn on(this: &NoUiSlider, event_name: &str, handler: &JsValue);
}
