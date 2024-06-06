mod string_manipulator;
mod wsl2_tutorial;
mod string_manipulation;
mod menu;
mod main_page;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::main_page::MainPage;
use crate::wsl2_tutorial::Wsl2Tutorial;
use crate::string_manipulator::StringManipulator;

fn main() {
    yew::Renderer::<RouterPage>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    MainPage,
    #[at("/wsl2")]
    Wsl2Tutorial,
    #[at("/stringmanipulator")]
    StringManipulator,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::MainPage => html! { <MainPage main_component={ html! { <h1>{ "Fab Dev" }</h1> } }/> },
        Route::Wsl2Tutorial => html! { <MainPage main_component={ html! { <Wsl2Tutorial /> } } /> },
        Route::StringManipulator => html! { <MainPage main_component={ html! { <StringManipulator /> } } /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(RouterPage)]
pub fn main_page() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}