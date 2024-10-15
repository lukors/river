mod edit_room_modal;

use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaComments;
use dioxus_free_icons::Icon;
use crate::room_data::{CurrentRoom, Rooms};
use self::edit_room_modal::EditRoomModal;
use ed25519_dalek::VerifyingKey;

#[component]
pub fn ChatRooms() -> Element {
    let rooms = use_context::<Signal<Rooms>>();
    let current_room = use_context::<Signal<CurrentRoom>>();
    let edit_modal_active = use_signal(|| None::<VerifyingKey>);

    let on_save = move |room_key: VerifyingKey, name: String, description: String| {
        rooms.with_mut(|rooms| {
            if let Some(room) = rooms.map.get_mut(&room_key) {
                room.room_state.configuration.configuration.name = name;
                room.room_state.configuration.configuration.description = description;
            }
        });
        edit_modal_active.set(None);
    };

    let on_cancel = move |_| {
        edit_modal_active.set(None);
    };
    rsx! {
        aside { class: "chat-rooms",
            div { class: "logo-container",
                img {
                    class: "logo",
                    src: "/freenet_logo.svg",
                    alt: "Freenet Logo"
                }
            }
            div { class: "sidebar-header",
                div { class: "rooms-title",
                    h2 {
                        Icon { icon: FaComments, width: 20, height: 20 }
                        span { "Rooms" }
                    }
                }
            }
            ul { class: "chat-rooms-list",
                {rooms.read().map.iter().map(|(room_key, room_data)| {
                    let room_key = *room_key;
                    let room_name = room_data.room_state.configuration.configuration.name.clone();
                    let is_current = current_room.read().owner_key == Some(room_key);
                    let mut current_room_clone = current_room.clone(); // Clone the Signal
                    rsx! {
                        li {
                            key: "{room_key:?}",
                            class: if is_current { "chat-room-item active" } else { "chat-room-item" },
                            div {
                                class: "room-item-container",
                                button {
                                    class: "room-name-button",
                                    onclick: move |_| {
                                        current_room_clone.set(CurrentRoom { owner_key : Some(room_key)});
                                    },
                                    "{room_name}"
                                }
                                button {
                                    class: "button is-small edit-room-button",
                                    onclick: move |_| {
                                        edit_modal_active.set(Some(room_key));
                                    },
                                    "Edit"
                                }
                            }
                        }
                    }
                }).collect::<Vec<_>>().into_iter()}
            }
        }
        EditRoomModal {
            active_room: edit_modal_active,
            on_save: Box::new(on_save),
            on_cancel: Box::new(on_cancel),
        }
    }
}
