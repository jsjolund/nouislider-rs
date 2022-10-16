use std::rc::Rc;

use gloo_utils::document;
use nouislider as no;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlDivElement;
use web_sys::HtmlElement;
use web_sys::Node;
use yew::prelude::*;

use super::dateslider::SliderUpdateRef;

// https://refreshless.com/nouislider/slider-options/#section-handle-attributes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HandleAttributes(pub Vec<HashMap<String, String>>);

// https://refreshless.com/nouislider/slider-values/
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Range(pub HashMap<String, Vec<f64>>);

// https://refreshless.com/nouislider/pips/
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pips {
    pub mode: String,
    pub density: Option<f64>,
    pub values: Option<Vec<f64>>,
    pub stepped: Option<bool>,
}

// These strings will overwrite default tooltips and pips
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct FormattedValues {
    pub tooltips_text: Vec<String>,
    pub pips_text: Vec<String>,
}

// https://refreshless.com/nouislider/slider-options/
#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub values: FormattedValues, // Parent override tooltips and pips
    pub start: Vec<f64>,
    pub range: Range,
    pub connect: Option<Vec<bool>>,
    pub step: Option<f64>,
    pub snap: Option<bool>,
    pub margin: Option<f64>,
    pub limit: Option<f64>,
    pub padding: Option<Vec<f64>>,
    pub orientation: Option<String>,
    pub direction: Option<String>,
    pub animate: Option<bool>,
    pub pips: Option<Pips>,
    pub tooltips: Option<bool>,
    pub behaviour: Option<String>,
    #[serde(rename = "handleAttributes")]
    pub handle_attributes: Option<HandleAttributes>,
    #[serde(rename = "keyboardSupport")]
    pub keyboard_support: Option<bool>,
    #[serde(rename = "keyboardDefaultStep")]
    pub keyboard_default_step: Option<f64>,
    #[serde(rename = "keyboardPageMultiplier")]
    pub keyboard_page_multiplier: Option<f64>,
    #[serde(rename = "keyboardMultiplier")]
    pub keyboard_multiplier: Option<f64>,
    #[serde(rename = "cssPrefix")]
    pub css_prefix: Option<String>,
    #[serde(rename = "cssClasses")]
    pub css_classes: Option<Vec<String>>,
}

// https://refreshless.com/nouislider/events-callbacks/#section-binding
// Slider updates sent to parent component
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    pub values: Vec<String>,
    pub handle: usize,
    pub unencoded: Vec<f64>,
    pub tap: bool,
    pub positions: Vec<f64>,
    pub pips: Vec<f64>,
}

pub struct Slider {
    _callbacks: Vec<Callback>,
    slider: no::NoUiSlider,
    container: HtmlElement,
    parent_state: Rc<SliderUpdateRef>,
    _listener: ContextHandle<Rc<SliderUpdateRef>>,
}

pub type JsVec = Vec<JsValue>;
pub type Callback = Closure<dyn Fn(JsVec, JsValue, JsVec, JsValue, JsVec, JsValue)>;

pub enum Msg {
    ContextChanged(Rc<SliderUpdateRef>),
}

fn get_pips(container: &HtmlElement) -> Vec<f64> {
    let pips_nodes = container.query_selector_all(".noUi-value").unwrap();
    let mut pips = vec![];
    for i in 0..pips_nodes.length() {
        let pip = pips_nodes
            .get(i)
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap()
            .get_attribute("data-value")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        pips.push(pip);
    }
    pips
}

impl Component for Slider {
    type Message = Msg;
    type Properties = Options;

    fn create(ctx: &Context<Self>) -> Self {
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("slider");

        let serailizer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);

        let slider = no::NoUiSlider::new(&container, &ctx.props().serialize(&serailizer).unwrap());

        let (parent_state, _listener) = ctx
            .link()
            .context::<Rc<SliderUpdateRef>>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");

        Self {
            slider,
            container,
            _callbacks: vec![],
            parent_state,
            _listener,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let parent_state = self.parent_state.clone();
            let container = self.container.clone();
            let callback = Callback::wrap(Box::new(
                move |values: Vec<JsValue>,
                      handle: JsValue,
                      unencoded: Vec<JsValue>,
                      tap: JsValue,
                      positions: Vec<JsValue>,
                      _nouislider: JsValue| {
                    // Construct event container
                    let update = Event {
                        values: values.into_iter().map(|v| v.as_string().unwrap()).collect(),
                        handle: handle.as_f64().unwrap() as usize,
                        unencoded: unencoded.into_iter().map(|v| v.as_f64().unwrap()).collect(),
                        tap: tap.as_bool().unwrap(),
                        positions: positions.into_iter().map(|v| v.as_f64().unwrap()).collect(),
                        pips: get_pips(&container), // Get pips too
                    };
                    // Forward slider update values to parent
                    let cb = parent_state.update.reform(move |update| (update));
                    cb.emit(update);
                },
            ));
            self.slider
                .on("update", callback.as_ref().dyn_ref().unwrap());
            self.slider.on("end", callback.as_ref().dyn_ref().unwrap());
            self._callbacks.push(callback);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(parent_state) => {
                self.parent_state = parent_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // TODO: This may be done with https://refreshless.com/nouislider/number-formatting/
        let pips = self.container.query_selector_all(".noUi-value").unwrap();
        for index in 0..pips.length() {
            let opt_text = ctx.props().values.pips_text.get(index as usize);
            if let Some(text) = opt_text {
                pips.get(index)
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap()
                    .set_text_content(Some(text));
            }
        }
        let tooltips = no::get_tooltips(&self.slider);
        for (index, tooltip) in tooltips.iter().enumerate() {
            let opt_text = ctx.props().values.tooltips_text.get(index);
            if let Some(text) = opt_text {
                tooltip.set_text_content(Some(text));
            }
        }
        let node: &Node = &self.container.clone().into();
        html! {
            {Html::VRef(node.clone())}
        }
    }
}
