mod freenet_api;

use super::{conversation::Conversation, members::MemberList, room_list::RoomList};
use crate::components::members::member_info_modal::MemberInfoModal;
use crate::components::room_list::edit_room_modal::EditRoomModal;
use crate::room_data::{CurrentRoom, Rooms};
use common::room_state::member::MemberId;
use dioxus::prelude::*;
use document::Stylesheet;
use ed25519_dalek::VerifyingKey;
use crate::components::app::freenet_api::FreenetApiSynchronizer;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(initial_rooms()));
    use_context_provider(|| Signal::new(CurrentRoom { owner_key: None }));
    use_context_provider(|| Signal::new(MemberInfoModalSignal { member: None }));
    use_context_provider(|| Signal::new(EditRoomModalSignal { room: None }));
    use_context_provider(|| Signal::new(CreateRoomModalSignal { show: false }));

    #[cfg(not(feature = "no-sync"))]
    {
        FreenetApiSynchronizer::start();
    }

    rsx! {
        Stylesheet { href: asset!("./assets/bulma.min.css") }
        Stylesheet { href: asset!("./assets/main.css") }
        Stylesheet { href: asset!("./assets/fontawesome/css/all.min.css") }
        div { class: "chat-container",
            RoomList {}
            Conversation {}
            MemberList {}
        }
        EditRoomModal {}
        MemberInfoModal {}

    }
}

#[cfg(not(feature = "example-data"))]
fn initial_rooms() -> Rooms {
    Rooms {
        map: std::collections::HashMap::new(),
    }
}

#[cfg(feature = "example-data")]
fn initial_rooms() -> Rooms {
    crate::example_data::create_example_rooms()
}

pub struct EditRoomModalSignal {
    pub room: Option<VerifyingKey>,
}

pub struct CreateRoomModalSignal {
    pub show: bool,
}

pub struct MemberInfoModalSignal {
    pub member: Option<MemberId>,
}
