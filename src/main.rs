use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let Some(session_name) = pipe_message.payload {
            switch_session(Some(&session_name));
        } else {
            eprintln!("no payload");
        }

        false
    }

    fn update(&mut self, _event: Event) -> bool {
        false
    }
}
