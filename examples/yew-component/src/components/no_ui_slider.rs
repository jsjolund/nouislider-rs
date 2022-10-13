use std::rc::Rc;

use gloo_utils::document;
use nouislider as no;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;
use yew::prelude::*;

use crate::AppState;
use crate::SliderUpdate;

#[derive(Properties, Clone, Eq, PartialEq)]
pub struct Props {
    pub start: Vec<i64>,
    pub connect: bool,
    pub range: no::Range,
    pub tooltips: bool,
}

pub struct Slider {
    _callbacks: Vec<no::Callback>,
    slider: no::NoUiSlider,
    container: HtmlElement,
    state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}

pub enum ChildMsg {
    ContextChanged(Rc<AppState>),
}

impl Component for Slider {
    type Message = ChildMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("slider");

        let opts = no::Options {
            start: ctx.props().start.clone(),
            connect: ctx.props().connect,
            range: ctx.props().range,
            tooltips: ctx.props().tooltips,
        };

        let slider = no::NoUiSlider::new_with_options(
            &container,
            &serde_wasm_bindgen::to_value(&opts).unwrap(),
        );

        let (state, _listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(ChildMsg::ContextChanged))
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
            let mousemove = no::Callback::wrap(Box::new(
                move |values: Vec<JsValue>,
                      handle: JsValue,
                      unencoded: Vec<JsValue>,
                      tap: JsValue,
                      positions: Vec<JsValue>,
                      _no_ui_slider: JsValue| {
                    let update = SliderUpdate {
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
                .on("update", mousemove.as_ref().dyn_ref().unwrap());
            self._callbacks.push(mousemove);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChildMsg::ContextChanged(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <br/>
            <br/>
                {self.render_slider()}
            </>
        }
    }
}

impl Slider {
    fn render_slider(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}
