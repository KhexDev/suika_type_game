use bevy::{prelude::*, audio::PlaybackMode};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{player::PlayerPlugin, container::ContainerPlugin, fruits::FruitsPlugin, mouse_pos::MousePosPlugin, next_fruits_ui::NextFruitUIPlugin};

mod player;
mod container;
mod fruits;
mod mouse_pos;
mod next_fruits_ui;
mod utils;

fn main() {
    println!("Hello, world!");
    App::new()
    .add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        // RapierDebugRenderPlugin::default(),
        PlayerPlugin,
        ContainerPlugin,
        FruitsPlugin,
        MousePosPlugin,
        NextFruitUIPlugin,
    ))
    .add_systems(Startup, (
        setup_cam,
        play_bg_music,
    ))
    .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn play_bg_music(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn(AudioBundle {
        source: assets.load("bg.mp3"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
        ..default()
    });
}