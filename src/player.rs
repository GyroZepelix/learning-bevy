use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Component, Input, KeyCode, Query, Res, Time, Transform};
use bevy::reflect::Reflect;
use bevy::prelude::ReflectComponent;
use bevy_inspector_egui::prelude::InspectorOptions;
use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (character_movement))
            .register_type::<Player>();
    }
}

#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut movement_amount = player.speed * time.delta_seconds();
        let mut movement_direction = Vec3::ZERO;

        input.get_pressed().for_each(|keycode| match keycode {
            KeyCode::W => movement_direction.y += 1.0,
            KeyCode::S => movement_direction.y -= 1.0,
            KeyCode::D => movement_direction.x += 1.0,
            KeyCode::A => movement_direction.x -= 1.0,
            KeyCode::ControlLeft => movement_amount *= 1.8,
            _ => {}
        });

        movement_direction = movement_direction.clamp_length(0.0,1.0);
        movement_direction *= movement_amount;

        transform.translation += movement_direction;
    }
}