use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use crate::{player::PlayerPlugin, container::ContainerPlugin, fruits::FruitsPlugin, mouse_pos::MousePosPlugin, next_fruits_ui::NextFruitUIPlugin, score::ScorePlugin, watermark::WatermarkPlugin, volume::VolumePlugin, background_music::BackgroundMusicPlugin};

mod player;
mod container;
mod fruits;
mod mouse_pos;
mod next_fruits_ui;
mod utils;
mod controls_window;
mod score;
mod watermark;
mod volume;
mod background_music;

use controls_window::*;

fn main() {
    println!("Hello, world!");
    App::new()

    .add_plugins((
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    title: "Goofy Suika Game".into(),
                    ..default()
                }),
                ..default()
            }
        ),
        // WorldInspectorPlugin::new(),
        EguiPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        // RapierDebugRenderPlugin::default(),
        ControlWindowPlugin,
        PlayerPlugin,
        ContainerPlugin,
        FruitsPlugin,
        MousePosPlugin,
        NextFruitUIPlugin,
        ScorePlugin,
        WatermarkPlugin,
        VolumePlugin,
        BackgroundMusicPlugin,
    ))
    .add_systems(Startup, (
        setup_cam,
        exit_game,
    ))
    .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn exit_game(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}