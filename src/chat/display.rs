use crate::{
    chat::logic::{ChatMessage, ChatState, PossibleResponces, Sender},
    colors::{COLORS, to_color},
    window::AppWindow,
};
use bevy::{input::mouse::MouseWheel, picking::hover::HoverMap, prelude::*};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .init_resource::<DropdownState>()
            .add_systems(Update, render_messages)
            .add_systems(
                Update,
                (
                    dropdown_toggle_system,
                    dropdown_option_system,
                    send_button_system,
                    sync_dropdown_ui,
                    maintain_dropdown_list,
                    auto_scroll_to_bottom.after(render_messages),
                ),
            )
            .add_systems(Update, send_scroll_events)
            .add_observer(on_scroll_handler);
    }
}

#[derive(Component)]
struct ScrollContainer;

#[derive(Component)]
struct MessageList;

#[derive(Component)]
struct SendButton;

#[derive(Component)]
struct DropdownTrigger;

#[derive(Component)]
struct DropdownPanel;

#[derive(Component)]
struct DropdownOptionButton(String);

#[derive(Component)]
struct DropdownLabel;

#[derive(Component)]
struct DropdownChevron;

#[derive(Resource, Default)]
struct DropdownState {
    selected: Option<String>,
    open: bool,
}

const BG: Color = to_color(COLORS.base);
const SIDEBAR_BG: Color = to_color(COLORS.mantle);

fn setup(mut commands: Commands, window: Query<&Window>) {
    let viewport_size = window
        .single()
        .unwrap()
        .resolution
        .physical_size()
        .as_vec2();

    commands
        .spawn((
            Node {
                width: Val::Percent(70.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Row,
                border_radius: BorderRadius::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow::clip(),

                ..default()
            },
            UiTransform::from_translation(Val2::px(0.3 * viewport_size.x, 0.05 * viewport_size.y)),
            BorderColor::all(to_color(COLORS.teal)),
            BackgroundColor(BG),
            AppWindow,
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                ..default()
            },))
                .with_children(|chat| {
                    chat.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            overflow: Overflow::scroll_y(),
                            ..default()
                        },
                        ScrollContainer,
                    ))
                    .with_children(|scroll| {
                        scroll.spawn((
                            MessageList,
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                padding: UiRect::all(Val::Px(16.0)),
                                ..default()
                            },
                        ));
                    });

                    chat.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(56.0),
                            flex_shrink: 0.0,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            column_gap: Val::Px(8.0),
                            border: UiRect::top(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(20.0)),
                            ..default()
                        },
                        BackgroundColor(SIDEBAR_BG),
                        BorderColor::all(to_color(COLORS.surface0)),
                    ))
                    .with_children(|bar| {
                        bar.spawn(Node {
                            flex_grow: 1.0,
                            height: Val::Px(36.0),
                            ..default()
                        })
                        .with_children(|wrapper| {
                            wrapper
                                .spawn((
                                    Button,
                                    DropdownTrigger,
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                                        border_radius: BorderRadius::all(Val::Px(18.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceBetween,
                                        border: UiRect::all(Val::Px(1.0)),
                                        overflow: Overflow::clip(),
                                        ..default()
                                    },
                                    BackgroundColor(to_color(COLORS.surface0)),
                                    BorderColor::all(to_color(COLORS.surface1)),
                                ))
                                .with_children(|trigger| {
                                    trigger.spawn((
                                        DropdownLabel,
                                        Text::new("Select a reply…"),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(to_color(COLORS.overlay0)),
                                    ));
                                    trigger.spawn((
                                        DropdownChevron,
                                        Text::new("V"),
                                        TextFont {
                                            font_size: 13.0,
                                            ..default()
                                        },
                                        TextColor(to_color(COLORS.subtext0)),
                                    ));
                                });

                            wrapper.spawn((
                                DropdownPanel,
                                Visibility::Hidden,
                                Node {
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(40.0),
                                    left: Val::Px(0.0),
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    border_radius: BorderRadius::all(Val::Px(12.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    padding: UiRect::all(Val::Px(4.0)),
                                    row_gap: Val::Px(2.0),
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                                BackgroundColor(to_color(COLORS.mantle)),
                                BorderColor::all(to_color(COLORS.surface1)),
                                GlobalZIndex(200),
                            ));
                        });

                        bar.spawn((
                            Button,
                            SendButton,
                            Node {
                                width: Val::Px(36.0),
                                height: Val::Px(36.0),
                                border_radius: BorderRadius::all(Val::Px(18.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_shrink: 0.0,
                                ..default()
                            },
                            BackgroundColor(to_color(COLORS.teal)),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("^"),
                                TextFont {
                                    font_size: 18.0,
                                    weight: FontWeight::BOLD,
                                    ..default()
                                },
                                TextColor(to_color(COLORS.base)),
                            ));
                        });
                    });
                });
        });
}

fn maintain_dropdown_list(
    option: Res<PossibleResponces>,
    mut commands: Commands,
    dp: Query<Entity, With<DropdownPanel>>,
) {
    if option.is_changed() {
        let mut dp = commands.entity(dp.single().unwrap());

        dp.despawn_children();

        dp.with_children(|panel| {
            for option in option.0.iter() {
                panel
                    .spawn((
                        Button,
                        DropdownOptionButton(option.0.clone()),
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|opt| {
                        opt.spawn((
                            Text::new(option.0.clone()),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(to_color(COLORS.text)),
                        ));
                    });
            }
        });
    }
}

fn dropdown_toggle_system(
    mut dropdown: ResMut<DropdownState>,
    mut trigger_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<DropdownTrigger>),
    >,
) {
    for (interaction, mut border) in &mut trigger_query {
        match *interaction {
            Interaction::Pressed => {
                dropdown.open = !dropdown.open;
                *border = if dropdown.open {
                    BorderColor::all(to_color(COLORS.teal))
                } else {
                    BorderColor::all(to_color(COLORS.surface1))
                };
            }
            Interaction::Hovered => {
                if !dropdown.open {
                    *border = BorderColor::all(to_color(COLORS.surface2));
                }
            }
            Interaction::None => {
                if !dropdown.open {
                    *border = BorderColor::all(to_color(COLORS.surface1));
                }
            }
        }
    }
}

fn dropdown_option_system(
    mut dropdown: ResMut<DropdownState>,
    mut option_query: Query<
        (&Interaction, &DropdownOptionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, opt, mut bg) in &mut option_query {
        match *interaction {
            Interaction::Pressed => {
                dropdown.selected = Some(opt.0.clone());
                dropdown.open = false;
                *bg = BackgroundColor(to_color(COLORS.surface0));
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(to_color(COLORS.surface0));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::NONE);
            }
        }
    }
}

fn sync_dropdown_ui(
    dropdown: Res<DropdownState>,
    mut panel_query: Query<&mut Visibility, With<DropdownPanel>>,
    mut label_query: Query<(&mut Text, &mut TextColor), With<DropdownLabel>>,
    mut chevron_query: Query<&mut Text, (With<DropdownChevron>, Without<DropdownLabel>)>,
) {
    if !dropdown.is_changed() {
        return;
    }

    if let Ok(mut vis) = panel_query.single_mut() {
        *vis = if dropdown.open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if let Ok(mut chevron) = chevron_query.single_mut() {
        **chevron = if dropdown.open {
            "^".into()
        } else {
            "V".into()
        };
    }

    if let Ok((mut text, mut color)) = label_query.single_mut() {
        if let Some(i) = &dropdown.selected {
            **text = i.clone();
            *color = TextColor(to_color(COLORS.text));
        } else {
            **text = "Select a reply…".into();
            *color = TextColor(to_color(COLORS.overlay0));
        }
    }
}

fn send_button_system(
    mut dropdown: ResMut<DropdownState>,
    mut message: MessageWriter<ChatMessage>,
    mut btn_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SendButton>),
    >,
) {
    for (interaction, mut bg) in &mut btn_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(i) = &dropdown.selected {
                    message.write(ChatMessage::new_now(i.clone(), Sender::You));
                    dropdown.selected = None;
                    dropdown.open = false;
                }
                *bg = BackgroundColor(to_color(COLORS.lavender));
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(to_color(COLORS.lavender));
            }
            Interaction::None => {
                *bg = BackgroundColor(to_color(COLORS.teal));
            }
        }
    }
}

fn render_messages(
    state: ResMut<ChatState>,
    mut commands: Commands,
    list_query: Query<Entity, With<MessageList>>,
) {
    if !state.is_changed() {
        return;
    }

    let messages = &state.0;
    let Ok(list_entity) = list_query.single() else {
        return;
    };

    commands.entity(list_entity).despawn_children();
    commands.entity(list_entity).with_children(|list| {
        for msg in messages.iter() {
            spawn_bubble(list, msg);
        }
    });
}

fn spawn_bubble(parent: &mut ChildSpawnerCommands, msg: &ChatMessage) {
    let justify = match msg.sender {
        Sender::You => JustifyContent::FlexEnd,
        _ => JustifyContent::FlexStart,
    };

    let message_color = to_color(match msg.sender {
        Sender::You => COLORS.teal,
        Sender::Ai => COLORS.red,
        Sender::John => COLORS.blue,
        Sender::Olivia => COLORS.mauve,
        Sender::Noah => COLORS.yellow,
    });

    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: justify,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Node {
                    max_width: Val::Percent(70.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    ..default()
                },
                BackgroundColor(message_color),
            ))
            .with_children(|bubble| {
                bubble.spawn((
                    Text::new(msg.to_string()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(to_color(COLORS.base)),
                    Node {
                        max_width: Val::Percent(100.0),
                        ..default()
                    },
                ));
            });
        });
}

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity,
    delta: Vec2,
}

fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
    let delta = &mut scroll.delta;

    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        let at_max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };
        if !at_max {
            scroll_position.x += delta.x;
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        let at_max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };
        if !at_max {
            scroll_position.y += delta.y;
            delta.y = 0.;
        }
    }

    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}

fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}
fn auto_scroll_to_bottom(
    state: Res<ChatState>,
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode), With<ScrollContainer>>,
) {
    if !state.is_changed() {
        return;
    }
    if let Ok((mut pos, computed)) = scroll_query.single_mut() {
        let max = computed.size().y * computed.inverse_scale_factor;

        pos.y = max.max(0.0);
    }
}
