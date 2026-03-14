use bevy::prelude::*;

use crate::{
    colors::{COLORS, to_color},
    terminal::logic::{Terminal, TerminalLine},
    window::{AppWindow, Focused},
};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (tick_cursor, refresh_ui))
            .init_resource::<CursorBlink>();
    }
}
#[derive(Resource)]
struct CursorBlink {
    timer: f32,

    visible: bool,
}

impl Default for CursorBlink {
    fn default() -> Self {
        Self {
            timer: 0.0,
            visible: true,
        }
    }
}
#[derive(Component)]
struct OutputRoot;

#[derive(Component)]
struct InputLine;

#[derive(Component)]
struct CursorNode;

#[derive(Component)]
pub struct TerminalWindow;

const BG: Color = to_color(COLORS.base);
const CURSOR_COLOR: Color = to_color(COLORS.sapphire);
const LINE_HEIGHT: f32 = 18.;
const FONT_SIZE: f32 = 16.;
const PROMPT_COLOR: Color = to_color(COLORS.text);

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                border_radius: BorderRadius::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            TerminalWindow,
            BorderColor::all(to_color(COLORS.sapphire)),
            BackgroundColor(BG),
            AppWindow,
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                OutputRoot,
                Node {
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::scroll_y(),
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            root.spawn((Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(4.0)),
                min_height: Val::Px(LINE_HEIGHT),
                ..default()
            },))
                .with_children(|row| {
                    row.spawn((
                        InputLine,
                        Text::new(""),
                        TextFont {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        TextColor(PROMPT_COLOR),
                    ));
                    row.spawn((
                        CursorNode,
                        Node {
                            width: Val::Px(9.0),
                            min_height: Val::Px(LINE_HEIGHT - 2.0),
                            margin: UiRect::left(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(CURSOR_COLOR),
                    ));
                });
        });
}
fn refresh_ui(
    term: Res<Terminal>,
    mut commands: Commands,
    output_q: Query<(Entity, &Node, &ComputedNode), With<OutputRoot>>,
    input_q: Query<Entity, With<InputLine>>,
    mut has_one_non_zero: Local<bool>,
) {
    if !term.is_changed() && *has_one_non_zero {
        return;
    }

    for (entity, _, computed_node) in output_q.iter() {
        commands.entity(entity).despawn_children();

        let visible_lines: Vec<&TerminalLine> = {
            let num = (computed_node.size.y / (2.0 * LINE_HEIGHT)).round() as usize;

            if num != 0 {
                *has_one_non_zero = true;
            }

            let start = term
                .lines
                .len()
                .saturating_sub(term.scroll_offset)
                .saturating_sub(num);

            term.lines[start..].iter().collect()
        };

        commands.entity(entity).with_children(|parent| {
            for line in &visible_lines {
                parent.spawn((
                    Text::new(&line.text),
                    TextFont {
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(to_color(COLORS.text)),
                    Node {
                        height: Val::Px(LINE_HEIGHT),
                        ..default()
                    },
                ));
            }
        });
    }

    for entity in input_q.iter() {
        let prompt = term.prompt_string();
        let before_cursor = &term.input[..term.cursor_pos];
        let display = format!("{}{}", prompt, before_cursor);
        commands
            .entity(entity)
            .insert(Text::new(display))
            .insert(TextColor(PROMPT_COLOR));
    }
}

const CURSOR_BLINK_RATE: f32 = 0.8;

fn tick_cursor(
    time: Res<Time>,
    mut blink: ResMut<CursorBlink>,
    mut cursor_q: Query<&mut BackgroundColor, With<CursorNode>>,
    term: Res<Terminal>,
    focused: Query<(), (With<Focused>, With<TerminalWindow>)>,
) {
    blink.timer += time.delta_secs();
    if blink.timer >= CURSOR_BLINK_RATE {
        blink.timer = 0.0;
        blink.visible = !blink.visible;
    }
    for mut bg in cursor_q.iter_mut() {
        *bg = if blink.visible && !focused.is_empty() {
            BackgroundColor(CURSOR_COLOR)
        } else {
            BackgroundColor(Color::NONE)
        };
    }
    if term.is_changed() {
        blink.visible = true;
        blink.timer = 0.0;
    }
}
