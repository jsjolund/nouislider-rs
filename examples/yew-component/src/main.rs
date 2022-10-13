use components::no_ui_slider::Slider;
use nouislider as no;
use nouislider_rs_example_yew_component::{AppState, Msg, SliderUpdate};
use std::rc::Rc;
use yew::prelude::*;

mod components;

struct Model {
    state: Rc<AppState>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let update = ctx.link().callback(Msg::Update);
        let state = Rc::new(AppState { update });
        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(update) => {
                log::debug!("parent {:?} {:?}", update.values, update.handle);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <ContextProvider<Rc<AppState>> context={self.state.clone()}>
                <Slider
                start={vec![20, 50]}
                connect={true}
                range={no::Range { min: 0, max: 100 }}
                tooltips={true}
                />
            </ContextProvider<Rc<AppState>>>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Model>::new().render();
}
