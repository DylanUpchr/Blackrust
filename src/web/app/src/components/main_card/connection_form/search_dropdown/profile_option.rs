use yew::{ prelude::*, html::Scope };
use stylist::css;

use crate::lib::Profile;
use super::SearchDropdown;

#[derive(Properties, PartialEq)]
pub struct ProfileOptionProps {
    pub profile: Profile
}

pub struct ProfileOption {
    parent: Option<Scope<SearchDropdown>>
}

impl Component for ProfileOption {
    type Message = ();
    type Properties = ProfileOptionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let parent: Option<Scope<SearchDropdown>>;
        let parent_link = ctx.link().get_parent();
        match parent_link {
            Some(connection_form) => {
                parent = Some(connection_form.clone().downcast::<SearchDropdown>());
            },
            None => parent = None
        }
        Self {
            parent: parent
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!("
        ");
        html! {
            <div {class}>
                {ctx.props().profile.name.clone()}
            </div>
        }
    }
}