use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use std::rc::Rc;
use yew::prelude::*;
use yew::Callback;

use crate::ParentRef;

use super::nouislider::Event as SliderEvent;
use super::nouislider::Range;
use super::nouislider::Slider;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Options {
    pub min: DateTime<FixedOffset>,
    pub max: DateTime<FixedOffset>,
}

#[derive(Clone, PartialEq)]
pub struct SliderUpdateRef {
    pub update: Callback<SliderEvent>,
}

pub enum Msg {
    ContextChanged(Rc<ParentRef>),
    SliderUpdate(SliderEvent),
}

pub struct DateSlider {
    parent_state: Rc<ParentRef>,
    _listener: ContextHandle<Rc<ParentRef>>,
    state: Rc<SliderUpdateRef>,
    slider_event: SliderEvent,
    slider_tooltip: Vec<String>,
    slider_0_start: i64,
    slider_1_start: i64,
    tz: FixedOffset,
}

fn from_timestamp(t: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(t, 0);
    naive.format("%Y-%m-%d %H:%M").to_string()
}

impl Component for DateSlider {
    type Message = Msg;
    type Properties = Options;

    fn create(ctx: &Context<Self>) -> Self {
        let update = ctx.link().callback(Msg::SliderUpdate);
        let state = Rc::new(SliderUpdateRef { update });

        let (parent_state, _listener) = ctx
            .link()
            .context::<Rc<ParentRef>>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");

        let min = ctx.props().min.timestamp();
        let max = ctx.props().max.timestamp();
        let slider_0_start = min + (max - min) / 2;
        let slider_1_start = max;
        Self {
            state,
            parent_state,
            _listener,
            slider_event: SliderEvent::default(),
            slider_0_start,
            slider_1_start,
            tz: ctx.props().min.timezone(),
            slider_tooltip: vec![
                from_timestamp(slider_0_start),
                from_timestamp(slider_1_start),
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SliderUpdate(event) => {
                self.slider_event = event;
                self.slider_tooltip.clear();
                let mut dates: Vec<DateTime<FixedOffset>> = vec![];

                for val in &self.slider_event.unencoded {
                    let ts = *val as i64;
                    let text = from_timestamp(ts);
                    self.slider_tooltip.push(text);

                    let naive = NaiveDateTime::from_timestamp(ts, 0)
                        .and_local_timezone(self.tz)
                        .unwrap();
                    dates.push(naive);
                }
                let cb = self.parent_state.update.reform(move |update| (update));
                cb.emit(dates);
                true
            }
            Msg::ContextChanged(state) => {
                self.parent_state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let min = ctx.props().min.timestamp();
        let max = ctx.props().max.timestamp();
        html! {
            <>
            <ContextProvider<Rc<SliderUpdateRef>> context={self.state.clone()}>
                <Slider
                    start={vec![self.slider_0_start, self.slider_1_start]}
                    connect={vec![false, true, false]}
                    range={Range { min, max }}
                    step={(max as f64 - min as f64)/1000.0}
                    tooltips={true}
                    tooltip_text={self.slider_tooltip.clone()}
                />
            </ContextProvider<Rc<SliderUpdateRef>>>
            // <br/>{self._draw_slider_data()}
            </>
        }
    }
}

impl DateSlider {
    fn _draw_slider_data(&self) -> Html {
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
                    <td>{"position"}</td>
                    {
                        self.slider_event.positions.clone().into_iter().map(|value| {
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
