use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew::Callback;

use crate::ParentRef;

use super::nouislider::Event as SliderEvent;
use super::nouislider::FormattedValues;
use super::nouislider::HandleAttributes;
use super::nouislider::Pips;
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
    slider_values: FormattedValues,
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
        Self {
            state,
            parent_state,
            _listener,
            tz: ctx.props().min.timezone(),
            slider_event: SliderEvent::default(),
            slider_values: FormattedValues::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SliderUpdate(event) => {
                self.slider_event = event;
                self.slider_values.tooltips_text.clear();
                self.slider_values.pips_text.clear();

                let mut dates: Vec<DateTime<FixedOffset>> = vec![];
                // Update the pips
                for val in &self.slider_event.pips {
                    let ts = *val as i64;
                    let text = from_timestamp(ts);
                    self.slider_values.pips_text.push(text);
                }

                // Update the tooltips
                for val in &self.slider_event.unencoded {
                    let ts = *val as i64;
                    let text = from_timestamp(ts);
                    self.slider_values.tooltips_text.push(text);

                    let naive = NaiveDateTime::from_timestamp(ts, 0)
                        .and_local_timezone(self.tz)
                        .unwrap();
                    dates.push(naive);
                }
                // Send values to parent component
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
        let min = (ctx.props().min.timestamp() - self.tz.utc_minus_local() as i64) as f64;
        let max = (ctx.props().max.timestamp() - self.tz.utc_minus_local() as i64 + 1) as f64;
        let delta = max - min;
        let range = HashMap::from([
            ("min".to_string(), vec![min]),
            ("max".to_string(), vec![max]),
            // ("20%".to_string(), vec![min + (delta * 0.01)]),
            // ("30%".to_string(), vec![min + (delta * 0.15)]),
            // ("40%".to_string(), vec![min + (delta * 0.40)]),
            // ("50%".to_string(), vec![min + (delta * 0.60)]),
            // ("60%".to_string(), vec![min + (delta * 0.9)]),
            // ("70%".to_string(), vec![min + (delta * 0.9)]),
        ]);
        let handle_attributes = vec![
            HashMap::from([("aria-label".to_string(), "lower".to_string())]),
            HashMap::from([("aria-label".to_string(), "upper".to_string())]),
        ];
        let pips = Pips {
            mode: "positions".to_string(),
            density: Some(1.0),
            values: Some(vec![0.0, 25.0, 50.0, 75.0, 100.0]),
            // stepped: None,
            ..Default::default()
        };
        let slider_start = vec![min + delta / 3.0, min + delta / 1.2];

        html! {
            <>
            <ContextProvider<Rc<SliderUpdateRef>> context={self.state.clone()}>
                <Slider
                    start={slider_start}
                    connect={vec![false, true, false]}
                    range={Range(range)}
                    pips={pips}
                    margin={delta/100.0}
                    handle_attributes={HandleAttributes(handle_attributes)}
                    tooltips={true}
                    values={self.slider_values.clone()}
                    // snap={true}
                    step={(max as f64 - min as f64)/100.0}
                    // limit={delta/1.3}
                    // padding={vec![delta/10.0, delta/15.0]}
                    // behaviour={"drag-fixed"}
                    // orientation={"vertical"}
                    // direction={"rtl"}
                    // keyboard_support={false}
                    // keyboard_default_step={100.0}
                />
            </ContextProvider<Rc<SliderUpdateRef>>>
            // {self._draw_slider_data()}
            </>
        }
    }
}

impl DateSlider {
    fn _draw_slider_data(&self) -> Html {
        let col0_width = format!("{:.0}%", 10f32);
        let col_width = format!("{:.0}%", 90f32 / self.slider_event.values.len() as f32);
        html! {
            <div class="my-6">
            <br />
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
            </div>
        }
    }
}
