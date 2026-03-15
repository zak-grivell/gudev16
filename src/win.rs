use bevy::prelude::*;

use crate::{
    colors::{COLORS, to_color},
    timer::GameTimer,
};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_popup)
            .init_resource::<WinState>();
    }
}

#[derive(Resource, Default)]
pub struct WinState(pub bool);

#[derive(Component)]
struct WinPopup;

fn spawn_popup(
    mut commands: Commands,
    connection: Res<WinState>,
    existing: Query<Entity, With<WinPopup>>,
    mut timer: Query<&mut GameTimer>,
) {
    if connection.0 && existing.is_empty() {
        timer.single_mut().unwrap().0.pause();
        commands
            .spawn((
                WinPopup,
                Node {
                    // Full-screen backdrop
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ZIndex(999),
                GlobalZIndex(1000),
            ))
            .with_children(|parent| {
                // Popup card
                parent
                    .spawn((
                        Node {
                            width: Val::Px(320.0),
                            height: Val::Auto,
                            padding: UiRect::all(Val::Px(32.0)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(16.0),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(to_color(COLORS.surface0)),
                        BorderColor::all(to_color(COLORS.red)),
                    ))
                    .with_children(|card| {
                        // Title
                        card.spawn((Text::new("You Stopped the AI"), TextColor(Color::WHITE)));

                        // Subtitle
                        card.spawn((
                            Text::new("For now the world is safe from AI slop"),
                            TextColor(to_color(COLORS.text)),
                        ));

                        // Reset button
                    });
            });
    } else if !connection.0 {
        for e in &existing {
            commands.entity(e).despawn();
        }
    }
}
