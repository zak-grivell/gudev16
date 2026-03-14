use bevy::prelude::*;

use crate::terminal::{display::DisplayPlugin, logic::LogicPlugin};

pub mod display;
pub mod logic;
pub mod tree;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LogicPlugin, DisplayPlugin));
    }
}
