use dioxus::prelude::*;
use crate::models::init_chat_state;

#[component]
pub fn ChatRooms(cx: Scope) -> Element {
    let chat_state = use_state(cx, init_chat_state);

    cx.render(rsx! {
        aside { class: "chat-rooms",
            h2 { class: "chat-rooms-title", "CHAT ROOMS" }
            ul { class: "chat-rooms-list",
                {chat_state.read().rooms.values().map(|room| {
                    let room_id = room.id.clone();
                    let room_name = room.name.clone();
                    let is_active = chat_state.read().current_room == room_id;
                    rsx! {
                        li {
                            key: "{room_id}",
                            class: if is_active { "active" } else { "" },
                            onclick: move |_| {
                                chat_state.write().current_room = room_id.clone();
                            },
                            "{room_name}"
                        }
                    }
                })}
            }
        }
    })
}
