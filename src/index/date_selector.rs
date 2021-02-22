
use yew::{html, Callback,  MouseEvent, Properties};
use yew_functional::function_component;
use yew_functional::{use_context};
use std::rc::Rc;
use crate::store::store::{ StoreModel};
use chrono::prelude::*;

use crate::components::header::Header;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onback: Callback<MouseEvent>,
    pub onselect: Callback<MouseEvent>,
}
#[function_component(DateSelector)]
pub fn date_selector(props: &Props) -> Html {
    let ctx = use_context::<Rc<StoreModel>>().expect("no ctx found");
    let StoreModel { date_selector_visible: show, .. } = &**ctx;

    let local_time = Local::now();
    let thw_month = local_time.month();
    let local_time1 = local_time.with_month(thw_month + 1).unwrap_or(local_time);
    let local_time2 = local_time.with_month(thw_month + 2).unwrap_or(local_time);

    let date_list: Vec<DateTime<Local>> = vec![local_time, local_time1, local_time2];

    let hidden_class = if *show { "" } else { "hidden" };

    let Props { onback, onselect } = &props;

    return html! {
        <div class=format!("date-selector {}", hidden_class) >
            <Header title="日期选择"
            onback=onback
            />
            <div class="date-selector-tables">
            {for date_list.iter().map(|date| {
                html! { <Month date=date onselect=onselect /> }
            })}
            </div>
        </div>
    };
}

#[derive(Properties, Clone, PartialEq)]
pub struct MonthProps {
    pub date: DateTime<Local>,
    pub onselect: Callback<MouseEvent>,
}
#[function_component(Month)]
fn month(props: &MonthProps) -> Html {
    let MonthProps { date, onselect } = &props;
    let the_month = date.month();
    let next_month_date = date.with_month(the_month + 1).unwrap_or(*date);
    let year_month = date.format("%Y年%m月").to_string();
    let first_day_in_month = date.with_day(1).unwrap_or(*date);
    let mut day_in_month = date.with_day(1).unwrap_or(*date);
    let mut day_list: Vec<Option<DateTime<Local>>> = vec![];

    let weekday = first_day_in_month.weekday();
    let offset = match weekday {
        chrono::Weekday::Mon => 0,
        chrono::Weekday::Tue => 1,
        chrono::Weekday::Wed => 2,
        chrono::Weekday::Thu => 3,
        chrono::Weekday::Fri => 4,
        chrono::Weekday::Sat => 5,
        chrono::Weekday::Sun => 6,
    };

    for _ in 0..offset {
        day_list.push(None);
    }

    // 先算出当月日期数组
    while the_month == day_in_month.month() {
        let the_day = day_in_month.day();
        day_list.push(Some(day_in_month));
        day_in_month = day_in_month.with_day(the_day + 1).unwrap_or(next_month_date);
    }

    let rest = day_list.iter().len() % 7;

    for _ in 0..(7 - rest) {
        day_list.push(None);
    }

    let len = day_list.iter().len();

    let rows = len / 7;

    let matrix: Vec<Vec<Option<DateTime<Local>>>> = (0..rows)
        .map(|i| {
            return (0..7)
                .map(|ii| {
                    let xy = i * 7 + ii;
                    return day_list[xy];
                })
                .collect();
        })
        .collect();


    return html! {
        <table class="date-table">
            <thead>
                <tr>
                    <td colSpan="7">
                        <h5>
                        {year_month}
                        </h5>
                    </td>
                </tr>
            </thead>
            <tbody>
                <tr class="data-table-weeks">
                    <th>{"周一"}</th>
                    <th>{"周二"}</th>
                    <th>{"周三"}</th>
                    <th>{"周四"}</th>
                    <th>{"周五"}</th>
                    <th class="weekend">{"周六"}</th>
                    <th class="weekend">{"周日"}</th>
                </tr>
                {for matrix.iter().map(|the_week| {
                    html! { <Week the_week=the_week onselect=onselect /> }
                })}
            </tbody>
        </table>
    };
}


#[derive(Properties, Clone, PartialEq)]
pub struct WeekProps {
    pub the_week: Vec<Option<DateTime<Local>>>,
    pub onselect: Callback<MouseEvent>,
}
#[function_component(Week)]
fn week(props: &WeekProps) -> Html {
    let WeekProps { the_week, onselect } = &props;

    return html! {
        <tr
        class="date-table-days"
        >
            {for the_week.iter().map(|date| {
                html! { <Day date=date onselect=onselect /> }
            })}
        </tr>
    };
}


#[derive(Properties, Clone, PartialEq)]
pub struct DayProps {
    pub date: Option<DateTime<Local>>,
    pub onselect: Callback<MouseEvent>,
}
#[function_component(Day)]
fn day(props: &DayProps) -> Html {
    let DayProps { date, onselect } = &props;
    let now = Local::now();

    let (the_day, day_str, is_today, prev_class, weekend_class) = match *date {
        Some(the_day) => {
            let weekend_class = match the_day.weekday() {
                chrono::Weekday::Sat | chrono::Weekday::Sun => "weekend",
                _ => "",
            };

            (
                the_day,
                the_day.day().to_string(),
                now.month() == the_day.month() && now.day() == the_day.day(),
                if the_day.day() < now.day() && now.month() == the_day.month() {
                    "disabled"
                } else {
                    ""
                },
                weekend_class,
            )
        }
        None => (Local::now(), "".to_string(), false, "", ""),
    };

    return html! {
        <td
        onclick=onselect
        class={format!("{} {}",weekend_class,prev_class)}
        >
            { if is_today { "今天".to_string() } else { day_str } }
        </td>
    };
}
