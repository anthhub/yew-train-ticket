
use yew::{html, Callback, Properties};
use yew_functional::function_component;
use crate::components::header::Header;
use yew_router::service::RouteService;
use urlparse::urlparse;
use urlparse::GetQuery;
use yew::web_sys;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[function_component(Query)]
pub fn query(_props: &Props) -> Html {
    let route_service: RouteService<String> = RouteService::new();
    let the_query = route_service.get_query();
    let url = urlparse(&the_query);
    let the_query = url.get_parsed_query().unwrap();
    let from: String = the_query.get_first_from_str("from").unwrap();
    let to: String = the_query.get_first_from_str("to").unwrap();
    let date: String = the_query.get_first_from_str("date").unwrap();
    let high_speed: String = the_query.get_first_from_str("high_speed").unwrap();

    let window = web_sys::window().unwrap();
    let history = window
        .history()
        .expect("browser does not support history API");

    let onback = Callback::from(move |_| {
        history.back().expect("back error");
        ()
    });

    return html! {
        <div class="header-wrapper">
             <Header title=format!("{} > {}",from,to) onback=onback />
             {date}{high_speed}
        </div>
    };
}