
use yew::{html, Callback,  MouseEvent, Properties};
use yew_functional::function_component;
use chrono::prelude::*;
use yew_functional::{use_context};
use std::rc::Rc;
use crate::store::store::{ StoreModel};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ontoggle: Callback<MouseEvent>,
}

#[function_component(DepartDate)]
pub fn depart_date(props: &Props) -> Html {
    let ctx = use_context::<Rc<StoreModel>>().expect("no ctx found");
    let StoreModel { local_time: date_time, .. } = &**ctx;

    let week = date_time.weekday();
    let time = date_time.format("%Y-%m-%d").to_string();
    let weekday_str = match week {
        chrono::Weekday::Mon => "周一",
        chrono::Weekday::Tue => "周二",
        chrono::Weekday::Wed => "周三",
        chrono::Weekday::Thu => "周四",
        chrono::Weekday::Fri => "周五",
        chrono::Weekday::Sat => "周六",
        chrono::Weekday::Sun => "周日",
    };

    let now = Local::now();

    let is_today = now.month() == date_time.month() && now.day() == date_time.day();

    let Props { ontoggle } = &props;

    return html! {
        <div class="depart-date"
            onclick=ontoggle
            >
                <input type="hidden" name="date"
                value=time
                />
                {time}
                <span class="depart-week">{weekday_str}{ if is_today {"(今天)"} else {""} }</span>
        </div>
    };
}

