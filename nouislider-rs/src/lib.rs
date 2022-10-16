use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use web_sys::HtmlElement;

pub fn get_tooltips(slider: &NoUiSlider) -> Vec<HtmlDivElement> {
    match js_sys::try_iter(&slider.get_tooltips()) {
        Ok(Some(iter)) => iter
            .map(|i| i.unwrap().dyn_into().unwrap())
            .collect::<Vec<HtmlDivElement>>(),
        _ => vec![],
    }
}

pub fn get_origins(slider: &NoUiSlider) -> Vec<HtmlDivElement> {
    match js_sys::try_iter(&slider.get_origins()) {
        Ok(Some(iter)) => iter
            .map(|i| i.unwrap().dyn_into().unwrap())
            .collect::<Vec<HtmlDivElement>>(),
        _ => vec![],
    }
}

pub fn get(slider: &NoUiSlider) -> Vec<f64> {
    slider
        .get()
        .into_iter()
        .map(|i| i.as_string().unwrap().parse::<f64>().unwrap())
        .collect::<Vec<f64>>()
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(js_name = create)]
    pub type NoUiSlider;

    // Create a new slider on target HtmlElement. Required options are 'start' and 'range'
    #[wasm_bindgen(constructor, js_class = create, js_namespace = noUiSlider)]
    pub fn new(target: &HtmlElement, options: &JsValue) -> NoUiSlider;

    // Removes classes from the root HtmlElement and empties it
    #[wasm_bindgen(method)]
    pub fn destroy(this: &NoUiSlider);

    // Set slider values
    #[wasm_bindgen(method)]
    pub fn set(this: &NoUiSlider, values: &JsValue);

    // Get slider values
    #[wasm_bindgen(method)]
    pub fn get(this: &NoUiSlider) -> Vec<JsValue>;

    // Get non-formatted values
    #[wasm_bindgen(method, js_name = get)]
    pub fn get_with_options(this: &NoUiSlider, no_formatting: bool) -> Vec<JsValue>;

    // Set callbacks for events: start, slide, drag, update, change, set, end
    #[wasm_bindgen(method)]
    pub fn on(this: &NoUiSlider, event_name: &str, handler: &JsValue);

    // Remove callbacks for events: start, slide, drag, update, change, set, end
    #[wasm_bindgen(method)]
    pub fn off(this: &NoUiSlider, event_name: &str);

    // Get slider handle origin HtmlDivElement
    #[wasm_bindgen(method, js_name = getOrigins)]
    pub fn get_origins(this: &NoUiSlider) -> JsValue;

    // Get slider positions
    #[wasm_bindgen(method, js_name = getPositions)]
    pub fn get_positions(this: &NoUiSlider) -> JsValue;

    // Get tooltips HtmlDivElements
    #[wasm_bindgen(method, js_name = getTooltips)]
    pub fn get_tooltips(this: &NoUiSlider) -> JsValue;

    // Remove tooltips
    #[wasm_bindgen(method, js_name = removeTooltips)]
    pub fn remove_tooltips(this: &NoUiSlider) -> JsValue;

    // Update options
    #[wasm_bindgen(method, js_name = updateOptions)]
    pub fn update_options(this: &NoUiSlider, option: &JsValue);

    // Set slider handle value, if 'set' event should fire, if stepping should be applied
    #[wasm_bindgen(method, js_name = setHandle)]
    pub fn set_handle(
        this: &NoUiSlider,
        handle_number: i64,
        value: f64,
        set_event: bool,
        stepping: bool,
    );

    // Set slider pips, return the HtmlDivElement
    #[wasm_bindgen(method, js_name = pips)]
    pub fn pips(this: &NoUiSlider, options: &JsValue) -> JsValue;

    // Remove pips
    #[wasm_bindgen(method, js_name = removePips)]
    pub fn remove_pips(this: &NoUiSlider);

    // Reset slider values
    #[wasm_bindgen(method)]
    pub fn reset(this: &NoUiSlider) -> JsValue;
}
