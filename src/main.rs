use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::{Camera2dBundle, Commands, ImagePlugin, KeyCode, Reflect, Res, Resource};
use bevy::render::camera::ScalingMode;
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::ReflectResource;
use crate::ai::AiPlugin;
use crate::pig::PigPlugin;
use crate::player::{Player, PlayerPlugin};
use crate::ui::GameUI;

mod pig;
mod ai;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Roguelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build()
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape))
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_plugins((
            PlayerPlugin,
            PigPlugin,
            AiPlugin,
            GameUI
        ))
        .register_type::<Money>()
        .run();
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player { speed: 96.0 },
        Name::new("Player")
    ));
}






