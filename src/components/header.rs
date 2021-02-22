
use yew::{html, Callback,  MouseEvent, Properties};
use yew_functional::function_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub onback: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let Props { title, onback } = &props;
    
    return html! {
        <div class="header">
            <div class="header-back"  style=" width=42 height=42" onclick=onback>
            {"<"}
            </div>
            <h1 class="header-title">{title}</h1>
        </div>
    };
}
