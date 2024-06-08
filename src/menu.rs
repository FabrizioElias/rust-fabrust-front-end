use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Menu)]
pub fn menu() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick_main_page = { let cloned = navigator.clone(); Callback::from(move |_| cloned.push(&Route::MainPage)) };
    let onclick_wsl2 = { let cloned = navigator.clone(); Callback::from(move |_| cloned.push(&Route::Wsl2Tutorial)) };
    let onclick_string_manipulator = { let cloned = navigator.clone(); Callback::from(move |_| cloned.push(&Route::StringManipulator)) };
    html! { 
        <div style="margin-bottom: 15px; padding-bottom: 15px;">
            <nav class="menu" id="nav">
                <span class="nav-item active">
                    <span class="icon">
                        <span class="subicon">{ 13 }</span>
                        <i data-feather="home"></i>
                    </span>
                    <a onclick={ onclick_main_page }>{ "Home" }</a>
                </span>
                <span class="nav-item">
                    <span class="icon">
                        <i data-feather="search"></i>
                    </span>
                    <a onclick={ onclick_wsl2 }>{ "WSL2 Tutorial" }</a>
                </span>
                <span class="nav-item">
                    <span class="icon">
                        <i data-feather="bell"></i>
                    </span>
                    <a onclick={ onclick_string_manipulator }>{ "String Manipulator" }</a>
                </span>
            </nav>
        </div>
    }
}