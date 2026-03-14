use std::sync::Arc;

use bevy::prelude::*;

pub struct AppWindowPlugin;

impl Plugin for AppWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_focus)
            .add_observer(handle_drag_start)
            .add_observer(handle_drag)
            .add_observer(handle_drag_drop)
            .add_observer(handle_click);
    }
}

#[derive(Component)]
#[require(Node, GlobalZIndex)]
pub struct AppWindow;

#[derive(Component)]
pub struct Focused;

fn on_focus(
    added: Query<Entity, Added<Focused>>,
    mut indexes: Query<(Entity, &mut GlobalZIndex), With<AppWindow>>,
    mut commands: Commands,
) {
    for new in &added {
        for (entity, mut z) in &mut indexes {
            if entity == new {
                z.0 = 100;
            } else {
                commands.entity(entity).remove::<Focused>();
                z.0 -= 1;
            }
        }
    }
}

fn handle_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    _query: Query<(), With<AppWindow>>,
) {
    commands.entity(trigger.event_target()).insert(Focused);
}

fn handle_drag_start(
    trigger: On<Pointer<DragStart>>,
    mut commands: Commands,
    mut query: Query<&mut UiTransform, With<AppWindow>>,
) {
    commands.entity(trigger.event_target()).insert(Focused);

    if let Ok(mut transform) = query.get_mut(trigger.event_target()) {
        transform.scale = Vec2::splat(1.0);
    }
}

fn handle_drag(
    trigger: On<Pointer<Drag>>,
    mut query: Query<&mut UiTransform, With<AppWindow>>,
    windows: Query<&Window>,
) {
    if let Ok(mut transform) = query.get_mut(trigger.event_target()) {
        let window = windows.single().unwrap();

        let viewport_size = window.resolution.physical_size().as_vec2();

        let d = trigger.event().delta;

        let t = match (transform.translation.x, transform.translation.y) {
            (Val::Px(x), Val::Px(y)) => Vec2::new(x, y),
            _ => panic!("not px"),
        };

        transform.translation = Val2::px(d.x + t.x, d.y + t.y);
    }
}

fn handle_drag_drop(
    trigger: On<Pointer<DragEnd>>,
    mut query: Query<&mut UiTransform, With<AppWindow>>,
) {
    if let Ok(mut transform) = query.get_mut(trigger.event_target()) {
        transform.scale = Vec2::splat(1.0);
    }
}
