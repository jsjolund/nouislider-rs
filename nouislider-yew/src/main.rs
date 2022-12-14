use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use components::dateslider::DateSlider;
use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;

pub mod components;

#[derive(Properties, Clone, PartialEq)]
pub struct ParentRef {
    pub update: Callback<Vec<DateTime<FixedOffset>>>,
}

pub enum Msg {
    DateUpdate(Vec<DateTime<FixedOffset>>),
}

struct Model {
    state: Rc<ParentRef>,
    dates: Vec<DateTime<FixedOffset>>,
    selected_dates: Vec<DateTime<FixedOffset>>,
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

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let min = FixedOffset::east(2 * 3600)
            .ymd(2000, 3, 29)
            .and_hms(12, 6, 43);
        let max = FixedOffset::east(2 * 3600)
            .ymd(2005, 12, 29)
            .and_hms(4, 29, 15);

        let dates = gen_random_dates(&min, &max, 500);

        let update = ctx.link().callback(Msg::DateUpdate);
        let state = Rc::new(ParentRef { update });

        Self {
            state,
            dates,
            selected_dates: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DateUpdate(dates_range) => {
                self.selected_dates = dates_range;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let output = match self.selected_dates.is_empty() {
            false => {
                let min = self.selected_dates.get(0).unwrap();
                let max = self.selected_dates.get(1).unwrap();
                let selected_count = self
                    .dates
                    .iter()
                    .filter_map(|date| match min <= date && date <= max {
                        true => Some(*date),
                        false => None,
                    })
                    .count();
                html! { <p>{""}{selected_count}{"/"}{self.dates.len()}{" dates in range"}</p> }
            }
            true => html! {},
        };

        let min = self.dates.first().unwrap();
        let max = self.dates.last().unwrap();
        html! {
            <div class="app">
            <div class="section mx-6">
            <ContextProvider<Rc<ParentRef>> context={self.state.clone()}>
                <DateSlider min={*min} max={*max} />
            </ContextProvider<Rc<ParentRef>>>
            </div>
            <div class="section mx-6">
            {output}
            </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Model>::new().render();
}
