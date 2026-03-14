use crate::{
    colors::{COLORS, to_color},
    window::AppWindow,
};
use bevy::{input::mouse::MouseWheel, picking::hover::HoverMap, prelude::*};

pub struct BrowserPlugin;

impl Plugin for BrowserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .init_resource::<Tabs>()
            .add_systems(Update, (render, tab_button_system))
            .add_systems(Update, send_scroll_events)
            .add_observer(on_scroll_handler);
    }
}

#[derive(Component)]
struct TabButton(usize);

#[derive(Resource)]
struct Tabs {
    tabs: Vec<Tab>,
    current_tab: usize,
    dirty: bool,
}

impl Default for Tabs {
    fn default() -> Self {
        Tabs {
            tabs: vec![
                Tab::new("Short Page".into(), "Just a little bit of text.".into()),
                Tab::new("Long Page".into(), "This page is very long...\n".repeat(50)),
            ],
            current_tab: 0,
            dirty: true,
        }
    }
}

impl Tabs {
    fn current_tab(&self) -> &Tab {
        &self.tabs[self.current_tab]
    }
}

struct Tab {
    name: String,
    content: Vec<PageContent>,
}

impl Tab {
    fn new(name: String, text: String) -> Self {
        Self {
            name: name.to_string(),
            content: vec![PageContent::Title(name), PageContent::Paragraph(text)],
        }
    }
}

enum PageContent {
    Title(String),
    Paragraph(String),
}

const BG: Color = to_color(COLORS.base);
const SIDEBAR_BG: Color = to_color(COLORS.mantle);

#[derive(Component)]
struct BrowserContent;

fn setup(mut commands: Commands, tabs: Res<Tabs>, window: Query<&Window>) {
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
                flex_direction: FlexDirection::Column,
                border_radius: BorderRadius::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(to_color(COLORS.mauve)),
            BackgroundColor(BG),
            UiTransform::from_translation(Val2::px(0.05 * viewport_size.x, 0.3 * viewport_size.y)),
            AppWindow,
            GlobalZIndex(100),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(45.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    // margin: UiRect::vertical(Val::Px(10.0)),
                    border_radius: BorderRadius::top(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(SIDEBAR_BG),
                BorderColor::all(to_color(COLORS.surface0)),
            ))
            .with_children(|parent| {
                for (i, tab) in tabs.tabs.iter().enumerate() {
                    parent
                        .spawn((
                            Button,
                            TabButton(i),
                            Node {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(4.0)),
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                justify_content: JustifyContent::Center,
                                height: Val::Px(25.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(to_color(COLORS.surface0)),
                        ))
                        .with_children(|b| {
                            b.spawn((
                                Text::new(&tab.name),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(to_color(COLORS.text)),
                            ));
                        });
                }
            });

            root.spawn(Node {
                width: Val::Percent(100.0),
                padding: UiRect::horizontal(Val::Px(15.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            })
            .with_children(|scroll_box| {
                scroll_box.spawn((
                    BrowserContent,
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::vertical(Val::Px(10.0)),

                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                ));
            });
        });
}

fn tab_button_system(
    mut tabs: ResMut<Tabs>,
    mut interaction_query: Query<
        (&Interaction, &TabButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, tab_index, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if tabs.current_tab != tab_index.0 {
                    tabs.current_tab = tab_index.0;
                    tabs.dirty = true;
                }
                *bg_color = BackgroundColor(to_color(COLORS.mauve));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(to_color(COLORS.overlay0));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(to_color(COLORS.surface0));
            }
        }
    }
}

fn render(
    mut browser: ResMut<Tabs>,
    mut commands: Commands,
    page_query: Query<Entity, With<BrowserContent>>,
) {
    if !browser.dirty {
        return;
    }
    browser.dirty = false;

    let tab = browser.current_tab();
    let Ok(page_entity) = page_query.single() else {
        return;
    };

    commands.entity(page_entity).despawn_children();

    commands.entity(page_entity).with_children(|p| {
        for content in &tab.content {
            match content {
                PageContent::Title(text) => {
                    p.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 32.0,
                            weight: FontWeight::BOLD,
                            ..default()
                        },
                        TextColor(to_color(COLORS.mauve)),
                    ));
                }
                PageContent::Paragraph(text) => {
                    p.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(to_color(COLORS.text)),
                        Node {
                            max_width: Val::Percent(100.0),
                            ..default()
                        },
                    ));
                }
            }
        }
    });
}

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity,
    /// Scroll delta in logical coordinates.
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
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
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
