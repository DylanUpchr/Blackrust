use yew::prelude::*;
use yew_feather::clock::Clock;
use stylist::css;
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
        let class = css!("
            display: flex;
            align-items: center;
            justify-content: center;

            p {
                display: inline;
                margin-left: 10px;
            }
        ");
        html! {
            <div {class}>
                <Clock color="black" size="30" />
                <p>{ now().format("%X") }</p>
            </div>
        }
    }
}

pub fn now() -> DateTime<Utc> {
    let now = js_sys::Date::new_0();
    DateTime::<Utc>::from(now)
}