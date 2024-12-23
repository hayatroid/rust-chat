use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            let web_socket_message: WebSocketMessage = serde_json::from_str(&ws_msg).unwrap();
            match web_socket_message.message_type {
                WebSocketMessageType::NewMessage => {
                    let msg = web_socket_message.message.expect("Missing message payload");
                    cloned_messages.push(msg);
                    messages_handle.set(cloned_messages);
                }
                WebSocketMessageType::UsersList => {}
            }
        }
    });

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_message = new_message.clone();
    let cloned_ws = ws.clone();
    let on_button_click = Callback::from(move |_: MouseEvent| {
        cloned_ws.send(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });

    html! {
        <div class="container">
            <div class="row">
                <div class="list-group">
                {
                    messages.iter().map(|m| html! {
                        <div class="list-group-item list-group-item-action">
                            <div class="d-flex w-100 justify-content-between">
                                <h5>{m.author.clone()}</h5>
                                <small>{m.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</small>
                            </div>
                            <p>{m.message.clone()}</p>
                        </div>
                    }).collect::<Html>()
                }
                </div>
            </div>
            <div class="row">
                <div class="input-group">
                    <textarea class="form-control" onchange={on_message_change} value={new_message}></textarea>
                    <button type="submit" class="btn btn-primary" onclick={on_button_click}>{"Send"}</button>
                </div>
            </div>
        </div>
    }
}
