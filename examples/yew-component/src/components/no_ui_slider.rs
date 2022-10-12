use gloo_utils::document;
use nouislider::NoUiSlider;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;
use yew::prelude::*;

#[derive(Properties, Clone, Eq, PartialEq)]
pub struct Props {}

#[derive(Serialize, Deserialize)]
struct SliderRange {
    min: i64,
    max: i64,
}
#[derive(Serialize, Deserialize)]
struct SliderOptions {
    start: Vec<i64>,
    connect: bool,
    range: SliderRange,
}

type JsVec = Vec<JsValue>;

type SliderCallback = Closure<dyn Fn(JsVec, JsValue, JsVec, JsValue, JsVec, JsValue)>;

pub struct Slider {
    _callbacks: Vec<SliderCallback>,
    slider: NoUiSlider,
    container: HtmlElement,
}

impl Component for Slider {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::debug!("create");
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("slider");

        let opts = SliderOptions {
            start: vec![20, 80],
            connect: true,
            range: SliderRange { min: 0, max: 100 },
        };

        log::debug!("{}", serde_json::to_string(&opts).unwrap());

        let slider =
            NoUiSlider::new_with_options(&container, &serde_wasm_bindgen::to_value(&opts).unwrap());

        let mousemove = SliderCallback::wrap(Box::new(
            move |values: Vec<JsValue>,
                  handle: JsValue,
                  unencoded: Vec<JsValue>,
                  tap: JsValue,
                  positions: Vec<JsValue>,
                  no_ui_slider: JsValue| {
                log::debug!("values {:?}, handle {:?}, unencoded {:?}, tap {:?}, positions {:?}, noUiSlider {:?}", values, handle, unencoded, tap, positions, no_ui_slider);
            },
        ));

        slider.on("update", mousemove.as_ref().dyn_ref().unwrap());

        let _callbacks = vec![mousemove];

        Self {
            slider,
            container,
            _callbacks,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        log::debug!("rendered");
        if first_render {
            log::debug!("{:?}", self.slider.get());
            log::debug!("{:?}", self.slider.get_with_options(true));
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::debug!("update");
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::debug!("view");
        html! {
            <>
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
