use yew::Callback;

pub mod components;

#[derive(Clone, Debug, PartialEq)]
pub struct SliderUpdate {
    pub values: Vec<String>,
    pub handle: usize,
    pub unencoded: Vec<f64>,
    pub tap: bool,
    pub positions: Vec<f64>,
}

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub update: Callback<SliderUpdate>,
}

pub enum Msg {
    Update(SliderUpdate),
}
