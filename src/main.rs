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
        Route::MainPage => html! { <MainPage main_component={ html! { <><h1>{ "Fab Dev" }</h1> <div>
        <p>{ "To run back end, execute in terminal the following command from inside /back-end:" }</p>
        <p>{ "cargo run" }</p>
        <p>{ "If you want live reloading, run:" }</p>
        <p>{ "cargo watch -c -q -x run" }</p>
        <p>{ "backend runs on localhost:3000" }</p>
        <p>{ "To run front end, execute in terminal the following command to enable live reloading from inside /front-end:" }</p>
        <p>{ "Install WASM target" }</p>
        <p>{ "rustup target add wasm32-unknown-unknown" }</p>
        <p>{ "Install trunk" }</p>
        <p>{ "cargo install --locked trunk" }</p>
        <p>{ "Serve the app" }</p>
        <p>{ "trunk serve" }</p>
        <p>{ "Serve the app and open the browser" }</p>
        <p>{ "trunk serve --open" }</p>
        <p>{ "frontend runs on localhost:8080 by default or whats configured on Trunk.toml" }</p>
    </div></> } }/> },
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