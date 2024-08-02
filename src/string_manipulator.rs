use std::error::{self, Error};

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

fn set_output_text_with_transform(input_string: &str, transform_function: fn (&str) -> Result<String, Box<dyn Error>>, element: NodeRef) {
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

fn set_output_text(input_result: &Result<String, Box<dyn error::Error>>, element: NodeRef) {
    let text_output_ref = element.clone();
    let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
    match input_result {
        Ok(input_string) => {
            if input_string.is_empty() {
                text_output_element.set_value("Type something into the input");
                return;
            }
            text_output_element.set_value(input_string);
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
    let hmac_key_ref = use_node_ref();
    let public_key_ref = use_node_ref();
    let private_key_ref = use_node_ref();

    let onclick_to_json = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, beautify_json, text_output_ref.clone())
        })
    };

    let onclick_beautify_json = {
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            //let text_output_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, beautify_json, text_output_ref.clone())
        })
    };

    let onclick_to_xml = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, beautify_xml, text_output_ref.clone())
        })
    };

    let onclick_beautify_xml = {
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, beautify_xml, text_output_ref.clone())
        })
    };

    let onclick_to_sql = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, beautify_sql, text_output_ref.clone())
        })
    };

    let onclick_beautify_sql = {
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, beautify_xml, text_output_ref.clone())
        })
    };

    let onclick_decode_unicode = {
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_output_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, decode_unicode_str, text_output_ref.clone())
        })
    };

    let onclick_to_decompress = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, decompress_and_decode_string, text_output_ref.clone())
        })
    };

    let onclick_to_decompress_hex = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, decompress_and_decode_hex_string, text_output_ref.clone())
        })
    };

    let onclick_to_compress = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, compress_string, text_output_ref.clone())
        })
    };

    let onclick_to_compress_hex = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, compress_string_hex, text_output_ref.clone())
        })
    };

    let onclick_to_xxhash = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, hash_string_xxhash, text_output_ref.clone())
        })
    };

    let onclick_to_sha256hash = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            set_output_text_with_transform(&input_string, hash_string, text_output_ref.clone())
        })
    };

    let onclick_sign_hmac = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        let hmac_key_ref = hmac_key_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let signing_key_element: HtmlTextAreaElement = hmac_key_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            let signing_key = signing_key_element.value();
            let result = sign_hmac(signing_key.as_bytes(), &input_string);
            set_output_text(&result, text_output_ref.clone())
        })
    };

    let onclick_encrypt_pkcs1 = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        let public_key_ref = public_key_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let public_key_element = public_key_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            let public_key = public_key_element.value();
            let result = encrypt_rsa_pkcs1(&public_key, &input_string);
            set_output_text(&result, text_output_ref.clone())
        })
    };

    let onclick_decrypt_pkcs1 = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        let private_key_ref = private_key_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let private_key_element = private_key_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            let private_key = private_key_element.value();
            let result = decrypt_rsa_pkcs1(&private_key, &input_string);
            set_output_text(&result, text_output_ref.clone())
        })
    };

    let onclick_encrypt_pkcs8 = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        let public_key_ref = public_key_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let public_key_element = public_key_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            let public_key = public_key_element.value();
            let result = encrypt_rsa_pkcs8(&public_key, &input_string);
            set_output_text(&result, text_output_ref.clone())
        })
    };

    let onclick_decrypt_pkcs8 = {
        let text_input_ref = text_input_ref.clone();
        let text_output_ref = text_output_ref.clone();
        let private_key_ref = private_key_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let text_input_element = text_input_ref.cast::<HtmlTextAreaElement>().unwrap();
            let private_key_element = private_key_ref.cast::<HtmlTextAreaElement>().unwrap();
            let input_string = text_input_element.value();
            let private_key = private_key_element.value();
            let result = decrypt_rsa_pkcs8(&private_key, &input_string);
            set_output_text(&result, text_output_ref.clone())
        })
    };

    html! {
        <main>
            <div>
                <div style="display: flex; justify-content: center; align-items: center;">
                    <div style="width: 50%; display: grid; align-self: flex-start;">
                        <textarea ref={ text_input_ref } style="flex: 1; margin-right: 10px;" rows="30"></textarea>
                        <div>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_json }>{ "To beauty JSON" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_xml }>{ "To beauty XML" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_sql }>{ "To beauty SQL" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_decompress }>{ "Decompress" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_decompress_hex }>{ "Decompress from hex (0x)" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_compress }>{ "Compress" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_compress_hex }>{ "Compress to hex (0x)" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_xxhash }>{ "Hash XXHash128" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_to_sha256hash }>{ "Hash SHA256" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_encrypt_pkcs1 }>{ "Encrypt RSA PKCS#1" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_decrypt_pkcs1 }>{ "Decrypt RSA PKCS#1" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_encrypt_pkcs8 }>{ "Encrypt PKCS#8" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_decrypt_pkcs8 }>{ "Decrypt PKCS#8" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_sign_hmac }>{ "Sign HMAC" }</button>
                        </div>
                    </div>
                    <div style="width: 50%; display: grid; align-self: flex-start;">
                        <textarea ref={ text_output_ref } style="flex: 1; margin-left: 10px;" rows="30" readonly=true></textarea>
                        <div style="margin-left: 10px;">
                            <button style="margin-bottom: 10px;" onclick={ onclick_beautify_json }>{ "Beautify JSON" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_beautify_xml }>{ "Beautify XML" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_beautify_sql }>{ "Beautify SQL" }</button>
                            <button style="margin-bottom: 10px;" onclick={ onclick_decode_unicode }>{ "Decode Unicode (\\u...)" }</button>
                        </div>
                    </div>
                </div>
                <div style="display: flex; flex-direction: column; max-width: 100%;">
                    <div>
                        <label for="hmac-key">{ "HMAC Key: "}</label>
                        <textarea ref={ hmac_key_ref } id="hmac-key" style="width: 100%;" rows="1"></textarea>
                    </div>
                    <div>
                        <label for="public-key">{ "Public Key (PEM RSA PKCS#1 or PKCS#8 format): "}</label>
                        <textarea ref={ public_key_ref } id="public-key" style="width: 100%;" rows="5"></textarea>
                    </div>
                    <div>
                        <label for="private-key">{ "Private Key (PEM RSA PKCS#1 or PKCS#8 format): "}</label>
                        <textarea ref={ private_key_ref } id="private-key" style="width: 100%;" rows="5"></textarea>
                    </div>
                </div>
            </div>
        </main>
    }
}