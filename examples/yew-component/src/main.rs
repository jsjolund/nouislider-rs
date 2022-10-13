use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use components::nouislider::Event;
use components::nouislider::Range;
use components::nouislider::Slider;
use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;
use yew::Callback;

pub mod components;

#[derive(Clone, PartialEq)]
pub struct ParentRef {
    pub update: Callback<Event>,
}

pub enum Msg {
    SliderUpdate(Event),
}

struct Model {
    state: Rc<ParentRef>,
    slider_event: Event,
    slider_tooltip: Vec<String>,
    dates: Vec<DateTime<FixedOffset>>,
}

fn gen_random_dates(
    min: &DateTime<FixedOffset>,
    max: &DateTime<FixedOffset>,
    num: usize,
) -> Vec<DateTime<FixedOffset>> {
    let mut rng = rand::thread_rng();
    let tz = min.timezone();
    let min_ = min.timestamp();
    let max_ = max.timestamp();
    let mut dates: Vec<DateTime<FixedOffset>> = (0..num - 2)
        .map(|_| {
            NaiveDateTime::from_timestamp(rng.gen_range(min_..max_), 0)
                .and_local_timezone(tz)
                .unwrap()
        })
        .collect();
    dates.push(*min);
    dates.push(*max);
    dates.sort();
    dates
}

fn from_timestamp(t: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(t, 0);
    naive.format("%Y-%m-%d %H:%M").to_string()
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let update = ctx.link().callback(Msg::SliderUpdate);
        let state = Rc::new(ParentRef { update });

        let min = FixedOffset::east(2 * 3600)
            .ymd(2017, 3, 29)
            .and_hms(12, 6, 43);
        let max = FixedOffset::east(2 * 3600)
            .ymd(2027, 12, 29)
            .and_hms(4, 29, 15);

        let dates = gen_random_dates(&min, &max, 500);

        Self {
            state,
            dates,
            slider_event: Event::default(),
            slider_tooltip: vec![
                from_timestamp(min.timestamp()),
                from_timestamp(max.timestamp()),
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SliderUpdate(event) => {
                self.slider_event = event;
                self.slider_tooltip.clear();
                for val in &self.slider_event.unencoded {
                    let text = from_timestamp(*val as i64);
                    self.slider_tooltip.push(text);
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let min = self.dates.first().unwrap().timestamp();
        let max = self.dates.last().unwrap().timestamp();
        html! {
            <div class="app">
            <div class="section">
            <ContextProvider<Rc<ParentRef>> context={self.state.clone()}>
                <Slider
                start={vec![max-(max-min)/4, max]}
                connect={vec![false, true, false]}
                range={Range { min, max }}
                step={1.0}
                tooltips={true}
                tooltip_text={self.slider_tooltip.clone()}
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
        let col_width = format!("{:.0}%", 90f32 / self.slider_event.values.len() as f32);
        html! {
            <table width="100%" class="table is-bordered is-narrow is-fullwidth">
            <tbody>
                <tr>
                    <td width={col0_width}>{"values"}</td>
                    {
                        self.slider_event.values.clone().into_iter().map(|value| {
                            html!{<td width={col_width.clone()}>{value}</td> }
                        }).collect::<Html>()
                    }
                </tr>
                <tr>
                    <td>{"unencoded"}</td>
                    {
                        self.slider_event.unencoded.clone().into_iter().map(|value| {
                            html!{<td>{value}</td> }
                        }).collect::<Html>()
                    }
                </tr>
                <tr>
                    <td>{"last handle"}</td>
                    <td>{self.slider_event.handle}</td>
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
