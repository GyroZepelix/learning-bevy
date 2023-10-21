use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Query, Reflect, Res, Time, Transform, Vec2};
use bevy::prelude::ReflectComponent;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, entity_wander_around)
            .register_type::<SimpleWanderAI>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SimpleWanderAI {
    pub direction: Vec2,
    pub speed: f32
}

fn entity_wander_around(
    mut entity: Query<(&mut Transform, &SimpleWanderAI)>,
    time: Res<Time>,
) {
    for (mut transform, ai_component) in &mut entity {
        let movement_amount = ai_component.speed * time.delta_seconds();
        let clamped_direction = ai_component.direction.clamp_length(1.0, 1.0);
        transform.translation += clamped_direction.extend(0.0) * movement_amount;
    }
}