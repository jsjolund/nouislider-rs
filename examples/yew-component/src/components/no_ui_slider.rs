use gloo_utils::document;
use nouislider::NoUiSlider;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;
use yew::prelude::*;

pub struct Slider {
    slider: NoUiSlider,
    container: HtmlElement,
}

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

impl Component for Slider {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::debug!("create");
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("slider");
        // let leaflet_map = noUiSlider::new_with_options(&container, &JsValue::NULL);
        let opts = SliderOptions {
            start: vec![20, 80],
            connect: true,
            range: SliderRange { min: 0, max: 100 },
        };
        // let opts_json = ;
        log::debug!("{}", serde_json::to_string(&opts).unwrap());

        let leaflet_map = NoUiSlider::new_with_options(
            &container,
            &serde_wasm_bindgen::to_value(&opts).unwrap(),
            // &JsValue::from_str(serde_json::to_string(&opts).unwrap().as_str()),
        );
        Self {
            slider: leaflet_map,
            container,
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
            <div class="map-container component-container">
                {self.render_slider()}
            </div>
        }
    }
}

impl Slider {
    fn render_slider(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}
