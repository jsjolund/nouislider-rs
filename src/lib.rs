use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

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
}
