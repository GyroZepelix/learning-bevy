use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::input::Input;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, default, Entity, KeyCode, Query, Reflect, Res, ResMut, SpatialBundle, SpriteBundle, Time, Timer, TimerMode, Transform, With};
use rand::Rng;
use bevy::prelude::ReflectComponent;
use crate::ai::SimpleWanderAI;
use crate::{Money};
use crate::player::Player;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pig, pig_lifetime))
            .add_systems(Startup, spawn_pig_parent)
            .register_type::<Pig>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>
) {
    if !input.just_pressed(KeyCode::Space) { return }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a pig, remaining money: ${:?}", money.0);

        let mut rng = rand::thread_rng();
        let texture = asset_server.load("pig.png");

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..default()
                },
                Pig {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
                SimpleWanderAI {
                    speed: 10.0,
                    direction: Vec2::new(rng.gen::<f32>()*2.0-1.0, rng.gen::<f32>()*2.0-1.0)
                }
            ));
        });
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    parent: Query<Entity, With<PigParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();

    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(parent).remove_children(&[pig_entity]);
            commands.entity(pig_entity).despawn();

            info!("Pig sold for $15! Current Money: ${:?}", money.0);
        }
    }
}

#[derive(Component)]
pub struct PigParent;

fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PigParent, Name::new("Pig Parent")));
}