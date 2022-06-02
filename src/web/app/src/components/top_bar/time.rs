use yew::prelude::*;
use yew_feather::clock::Clock;
use stylist::css;
use instant::Instant;

pub struct Time {
    now: Instant
}

pub enum TimeMsg {
    UpdateClock
}

impl Component for Time {
    type Message = TimeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            now: Instant::now()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UpdateClock => {
                self.now = Instant::now();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div><Clock />{ self.now.format("%X") }</div>
        }
    }
}