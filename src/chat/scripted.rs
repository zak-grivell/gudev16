use bevy::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};

use crate::chat::logic::{ChatMessage, ChatState};
use crate::timer::{GameTimer, TimeUp};

#[derive(Resource)]
struct ChatHandl(Handle<LoadedCsv<ChatMessage>>);

pub struct ScriptedPlugin;

impl Plugin for ScriptedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_plugins((CsvAssetPlugin::<ChatMessage>::new(&["scripted_text.csv"]),))
            .add_systems(Update, (timed_chat_system, on_reset))
            .init_resource::<NextIndex>();
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = ChatHandl(asset_server.load("scripted_text.csv"));
    commands.insert_resource(level);
}

#[derive(Resource, Default)]
struct NextIndex(usize);

fn timed_chat_system(
    chat_handle: Res<ChatHandl>,
    csv_assets: Res<Assets<LoadedCsv<ChatMessage>>>,
    timer: Query<&GameTimer>,
    mut next_index: ResMut<NextIndex>,
    mut chat_state: ResMut<ChatState>,
) {
    let t = timer.single().unwrap();

    if let Some(csv) = csv_assets.get(&chat_handle.0) {
        let elapsed = t.0.elapsed_secs();

        while next_index.0 < csv.rows.len() {
            let msg = &csv.rows[next_index.0];

            let t = msg.time.unwrap();

            if elapsed >= t {
                chat_state.0.push(msg.clone());

                // println!("{}: {}", msg.sender, msg.message);
                next_index.0 += 1;
            } else {
                break;
            }
        }
    }
}
fn on_reset(mut game_over: MessageReader<TimeUp>, mut state: ResMut<NextIndex>) {
    for _ in game_over.read() {
        state.0 = 0;
    }
}
