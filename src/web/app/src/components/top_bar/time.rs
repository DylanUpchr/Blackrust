use yew::prelude::*;
use yew_feather::clock::Clock;
use chrono::{DateTime, Utc};
use gloo_timers::callback::Interval;

pub struct Time {
    interval: Interval
}

pub enum TimeMsg {
    UpdateClock
}

impl Component for Time {
    type Message = TimeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let interval = {
            let link = ctx.link().clone();
            Interval::new(250, move || link.send_message(TimeMsg::UpdateClock))
        };
        Self {
            interval: interval
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Clock />
                { now().format("%X") }
            </div>
        }
    }
}

pub fn now() -> DateTime<Utc> {
    let now = js_sys::Date::new_0();
    DateTime::<Utc>::from(now)
}