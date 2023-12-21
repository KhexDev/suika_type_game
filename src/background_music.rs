use bevy::{prelude::*, audio::PlaybackMode};

use crate::volume::GameVolume;

pub struct BackgroundMusicPlugin;

impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_bg_music);
        app.add_systems(Update, update_volume);
    }
}

#[derive(Component)]
pub struct BackgroundMusic;

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
    })
    .insert(BackgroundMusic);
}

fn update_volume(
    q: Query<&AudioSink, With<BackgroundMusic>>,
    game_volume: Res<GameVolume>,
) {
    if game_volume.is_changed() {
        if let Ok(sink) = q.get_single() {
            sink.set_volume(game_volume.bg_volume);    
        }
    }
}