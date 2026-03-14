use bevy::{asset::load_internal_binary_asset, prelude::*, window::WindowResolution};

use crate::{
    browser::BrowserPlugin, chat::ChatPlugin, terminal::TerminalPlugin, timer::TimePlugin,
    window::AppWindowPlugin,
};

mod browser;
mod chat;
mod colors;
mod terminal;
mod timer;
mod window;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Push to Prod".into(),
            resolution: WindowResolution::new(1080, 720),
            resizable: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(AppWindowPlugin)
    .add_plugins((TerminalPlugin, BrowserPlugin, ChatPlugin, TimePlugin))
    .add_systems(Startup, setup);

    let text_font = TextFont::default();

    load_internal_binary_asset!(
        app,
        text_font.font,
        "../assets/JetBrainsMono-VariableFont_wght.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}
