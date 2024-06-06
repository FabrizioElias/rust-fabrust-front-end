use yew::prelude::*;

use crate::menu::Menu;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub main_component: Html,
}

#[function_component(MainPage)]
pub fn main_page(props: &Props) -> Html {
    html! {
        <>
            <Menu />
            { props.main_component.clone() }
        </>
    }
}