use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

use crate::{
    chat::logic::ChatMessage,
    colors::{COLORS, to_color},
    window::{AppWindow, Focused},
};

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (handle_keyboard, tick_cursor, refresh_ui, command_handler),
            )
            .init_resource::<Terminal>()
            .add_message::<Command>()
            .init_resource::<CursorBlink>();
    }
}
