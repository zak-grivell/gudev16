use bevy::prelude::*;

use crate::chat::display::DisplayPlugin;
use crate::chat::logic::LogicPlugin;
use crate::chat::scripted::ScriptedPlugin;

pub mod display;
pub mod logic;
pub mod scripted;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DisplayPlugin, LogicPlugin, ScriptedPlugin));
    }
}
