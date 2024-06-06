use std::error::Error;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::string_manipulation::*;
/*
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

async fn get_user(username: &str) -> Result<User, ApiError> {
    let create_user = CreateUser { username: username.to_string() };
    let request = Request::post("http://localhost:3000/users").json(&create_user);
    match request {
        Ok(request) => {
            let sent_request = request.send().await;
            match sent_request {
                Ok(response) => {
                    let json = response.json().await;
                    match json {
                        Ok(user) => user,
                        Err(e) => Err(ApiError { id: 0, message: e.to_string() })
                    }
                },
                Err(e) => Err(ApiError { id: 0, message: e.to_string() }),
            }
        },
        Err(e) => Err(ApiError { id: 0, message: e.to_string() })
    }

    
#[derive(Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize, Clone)]
struct ApiError {
    id: u64,
    message: String,
}

#[derive(Serialize)]
struct CreateUser {
    username: String,
}

    let api_error: UseStateHandle<Option<ApiError>> = use_state(|| None);
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    //wsl2_ref.cast::<HtmlElement>().unwrap().set_class_name("hidden-item");

    let user = use_state(|| User { id: 0, username: "".to_string() });
    let onclick_post = {
        let user_fn = user.clone();
        let api_error_fn = api_error.clone();
        move |_| {
            let user_closure = user_fn.clone();
            let api_error_closure = api_error_fn.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_user("name").await {
                    Ok(inner_fetched) => user_closure.set(inner_fetched),
                    Err(e) => api_error_closure.set(Some(e))
                }
            });
        }
    };
}*/

fn set_output_text(input_string: &str, transform_function: fn (&str) -> Result<String, Box<dyn Error>>, element: NodeRef) {
    let text_output_ref = element.clone();
    let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
    if input_string.is_empty() {
        text_output_element.set_value("Type something into the input");
        return;
    }
    let output_string = transform_function(input_string);
    match output_string {
        Ok(output_string) => {
            text_output_element.set_value(&output_string);
        },
        Err(e) => {
            text_output_element.set_value(&e.to_string());
        }
    }
}

#[function_component(StringManipulator)]
pub fn string_manipulator() -> Html {
    let text_input_ref = use_node_ref();
    let text_output_ref = use_node_ref();

    let onclick_to_json = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, beautify_json, text_output_ref.clone())
        })
    };

    let onclick_to_xml = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, beautify_xml, text_output_ref.clone())
        })
    };

    let onclick_to_sql = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, beautify_sql, text_output_ref.clone())
        })
    };

    let onclick_to_decompress = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, decompress_and_decode_string, text_output_ref.clone())
        })
    };

    let onclick_to_compress = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, compress_string, text_output_ref.clone())
        })
    };

    let onclick_to_xxhash = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, hash_string_xxhash, text_output_ref.clone())
        })
    };

    let onclick_to_sha256hash = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text(&input_string, hash_string, text_output_ref.clone())
        })
    };

    html! {
        <main>
            <div>
            <div style="display: flex; justify-content: space-between; align-items: center;">
                <textarea ref={ text_input_ref } style="flex: 1; margin-right: 10px;" rows="10"></textarea>
                <div style="display: flex; flex-direction: column; justify-content: center; align-items: center;">
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_json }>{ "To beauty JSON" }</button>
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_xml }>{ "To beauty XML" }</button>
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_sql }>{ "To beauty SQL" }</button>
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_decompress }>{ "Decompress" }</button>
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_compress }>{ "Compress" }</button>
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_xxhash }>{ "Hash XXHash128" }</button>
                    <button style="margin-bottom: 10px;" onclick={ onclick_to_sha256hash }>{ "Hash SAH256" }</button>
                </div>
                <textarea ref={ text_output_ref } style="flex: 1; margin-left: 10px;" rows="10" readonly=true></textarea>
            </div>
            </div>
        </main>
    }
}