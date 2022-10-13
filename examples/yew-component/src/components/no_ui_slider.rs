use std::rc::Rc;

use gloo_utils::document;
use nouislider as no;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;
use yew::prelude::*;

use crate::ParentRef;

#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub start: Vec<i64>,
    pub connect: Vec<bool>,
    pub range: Range,
    pub step: f64,
    pub tooltips: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    pub values: Vec<String>,
    pub handle: usize,
    pub unencoded: Vec<f64>,
    pub tap: bool,
    pub positions: Vec<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub min: i64,
    pub max: i64,
}

pub struct Slider {
    _callbacks: Vec<Callback>,
    slider: no::NoUiSlider,
    container: HtmlElement,
    state: Rc<ParentRef>,
    _listener: ContextHandle<Rc<ParentRef>>,
}

pub type JsVec = Vec<JsValue>;
pub type Callback = Closure<dyn Fn(JsVec, JsValue, JsVec, JsValue, JsVec, JsValue)>;

pub enum Msg {
    ContextChanged(Rc<ParentRef>),
}

impl Component for Slider {
    type Message = Msg;
    type Properties = Options;

    fn create(ctx: &Context<Self>) -> Self {
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("slider");

        let slider = no::NoUiSlider::new(
            &container,
            &serde_wasm_bindgen::to_value(&ctx.props()).unwrap(),
        );

        let (state, _listener) = ctx
            .link()
            .context::<Rc<ParentRef>>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");

        Self {
            slider,
            container,
            _callbacks: vec![],
            state,
            _listener,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let state = self.state.clone();
            let callback = Callback::wrap(Box::new(
                move |values: Vec<JsValue>,
                      handle: JsValue,
                      unencoded: Vec<JsValue>,
                      tap: JsValue,
                      positions: Vec<JsValue>,
                      _no_ui_slider: JsValue| {
                    let update = Event {
                        values: values.into_iter().map(|v| v.as_string().unwrap()).collect(),
                        handle: handle.as_f64().unwrap() as usize,
                        unencoded: unencoded.into_iter().map(|v| v.as_f64().unwrap()).collect(),
                        tap: tap.as_bool().unwrap(),
                        positions: positions.into_iter().map(|v| v.as_f64().unwrap()).collect(),
                    };
                    let cb = state.update.reform(move |update| (update));
                    cb.emit(update);
                },
            ));
            self.slider
                .on("update", callback.as_ref().dyn_ref().unwrap());
            self._callbacks.push(callback);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            {self.render_slider()}
        }
    }
}

impl Slider {
    fn render_slider(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}
