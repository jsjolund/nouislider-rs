use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use web_sys::HtmlElement;

pub fn get_tooltips_elements(slider: &NoUiSlider) -> Vec<HtmlDivElement> {
    match js_sys::try_iter(&slider.get_tooltips()) {
        Ok(Some(iter)) => iter
            .map(|i| i.unwrap().dyn_into().unwrap())
            .collect::<Vec<HtmlDivElement>>(),
        _ => vec![],
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(js_name = create)]
    pub type NoUiSlider;

    #[wasm_bindgen(constructor, js_class = create, js_namespace = noUiSlider)]
    pub fn new(target: &HtmlElement, options: &JsValue) -> NoUiSlider;

    // Get slider values
    #[wasm_bindgen(method)]
    pub fn get(this: &NoUiSlider) -> Vec<JsValue>;

    // Set slider values
    #[wasm_bindgen(method)]
    pub fn set(this: &NoUiSlider, values: &JsValue);

    #[wasm_bindgen(method, js_name = get)]
    pub fn get_with_options(this: &NoUiSlider, no_formatting: bool) -> Vec<JsValue>;

    #[wasm_bindgen(method)]
    pub fn on(this: &NoUiSlider, event_name: &str, handler: &JsValue);

    #[wasm_bindgen(method, js_name = getOrigins)]
    pub fn get_origins(this: &NoUiSlider) -> JsValue;

    #[wasm_bindgen(method, js_name = getTooltips)]
    pub fn get_tooltips(this: &NoUiSlider) -> JsValue;

}
