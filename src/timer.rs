use std::time::Duration;

use bevy::prelude::*;

use crate::colors::{COLORS, to_color};

#[derive(Message, Default)]
pub struct TimeUp;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    timer_update,
                    update_timer_text,
                    spawn_popup,
                    handle_reset_button,
                ),
            )
            .init_resource::<ConnectionState>()
            .add_message::<TimeUp>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(GameTimer(Timer::new(
        Duration::from_mins(3),
        TimerMode::Repeating,
    )));

    commands.spawn((
        Text::new("0.00s"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            right: Val::Px(16.0),
            ..default()
        },
        TimerText,
    ));
}

#[derive(Component)]
pub struct GameTimer(pub Timer);

#[derive(Component)]
struct TimerText;

fn timer_update(
    time: Res<Time>,
    mut timer: Query<&mut GameTimer>,
    mut dc: ResMut<ConnectionState>,
) {
    let mut t = timer.single_mut().unwrap();
    let d = time.delta();

    if t.0.remaining() <= d {
        t.0.pause();

        dc.disconnected = true;
    } else {
        t.0.tick(d);
    };
}

fn update_timer_text(timer: Query<&GameTimer>, mut query: Query<&mut Text, With<TimerText>>) {
    for mut text in &mut query {
        let d = timer.single().unwrap().0.elapsed().as_secs();

        **text = format!("16:{}:{:02}", (d / 60) + 57, d % 60);
    }
}

// --- Components & State ---

#[derive(Component)]
struct DisconnectedPopup;

#[derive(Component)]
struct ResetButton;

#[derive(Resource, Default)]
pub struct ConnectionState {
    pub disconnected: bool,
}

fn spawn_popup(
    mut commands: Commands,
    connection: Res<ConnectionState>,
    existing: Query<Entity, With<DisconnectedPopup>>,
) {
    if connection.disconnected && existing.is_empty() {
        commands
            .spawn((
                DisconnectedPopup,
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
                        card.spawn((Text::new("Disconnected"), TextColor(Color::WHITE)));

                        // Subtitle
                        card.spawn((
                            Text::new("Connection to the server was lost."),
                            TextColor(to_color(COLORS.text)),
                        ));

                        // Reset button
                        card.spawn((
                            ResetButton,
                            Button,
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(42.0),
                                margin: UiRect::top(Val::Px(8.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(to_color(COLORS.red)),
                        ))
                        .with_children(|btn| {
                            btn.spawn((Text::new("Reset"), TextColor(to_color(COLORS.base))));
                        });
                    });
            });
    } else if !connection.disconnected {
        for e in &existing {
            commands.entity(e).despawn();
        }
    }
}

fn handle_reset_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut connection: ResMut<ConnectionState>,
    mut time_up: MessageWriter<TimeUp>,
    mut timer: Query<&mut GameTimer>,
) {
    for (interaction, mut bg) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                connection.disconnected = false;
                time_up.write_default();

                let mut t = timer.single_mut().unwrap();

                t.0.unpause();
                t.0.reset();
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(to_color(COLORS.maroon));
            }
            Interaction::None => {
                *bg = BackgroundColor(to_color(COLORS.red));
            }
        }
    }
}
