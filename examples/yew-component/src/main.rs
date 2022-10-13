use components::no_ui_slider::Event;
use components::no_ui_slider::Range;
use components::no_ui_slider::Slider;
use std::rc::Rc;
use yew::prelude::*;
use yew::Callback;

pub mod components;

#[derive(Clone, PartialEq)]
pub struct ParentRef {
    pub update: Callback<Event>,
}

pub enum Msg {
    Update(Event),
}

struct Model {
    state: Rc<ParentRef>,
    slider: Event,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let update = ctx.link().callback(Msg::Update);
        let state = Rc::new(ParentRef { update });
        Self {
            state,
            slider: Event::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(update) => {
                self.slider = update;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
            <div class="section">
            <ContextProvider<Rc<ParentRef>> context={self.state.clone()}>
                <Slider
                start={vec![20, 60, 80, 90]}
                connect={vec![false, true, false, true, false]}
                range={Range { min: 0, max: 200 }}
                step={0.000001}
                tooltips={true}
                />
            </ContextProvider<Rc<ParentRef>>>
            </div>
            <div class="section">
            {self.draw_slider_data()}
            </div>
            </div>
        }
    }
}

impl Model {
    fn draw_slider_data(&self) -> Html {
        let col0_width = format!("{:.0}%", 10f32);
        let col_width = format!("{:.0}%", 90f32 / self.slider.values.len() as f32);
        html! {
            <table width="100%" class="table is-bordered is-narrow is-fullwidth">
            <tbody>
                <tr>
                    <td width={col0_width}>{"values"}</td>
                    {
                        self.slider.values.clone().into_iter().map(|value| {
                            html!{<td width={col_width.clone()}>{value}</td> }
                        }).collect::<Html>()
                    }
                </tr>
                <tr>
                    <td>{"unencoded"}</td>
                    {
                        self.slider.unencoded.clone().into_iter().map(|value| {
                            html!{<td>{value}</td> }
                        }).collect::<Html>()
                    }
                </tr>
                <tr>
                    <td>{"last handle"}</td>
                    <td>{self.slider.handle}</td>
                </tr>
            </tbody>
            </table>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Model>::new().render();
}
