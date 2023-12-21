use bevy::prelude::*;
use bevy_inspector_egui::{
    egui::{self, Pos2},
    bevy_egui::EguiContexts,
};

pub struct VolumePlugin;

impl Plugin for VolumePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameVolume>();
        app.add_systems(Update, draw_window);
    }
}

#[derive(Resource)]
pub struct GameVolume {
    pub bg_volume: f32,
    pub sfx_volume: f32,
}

impl Default for GameVolume {
    fn default() -> Self {
        GameVolume {
            bg_volume: 1.0,
            sfx_volume: 1.0,
        }
    }
}

fn draw_window(
    mut contexts: EguiContexts,
    mut game_volume: ResMut<GameVolume>,
) {
    egui::Window::new("Volume")
    .default_pos(Pos2::new(0.0, 100.0))
    .show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut game_volume.bg_volume, 0.0..=1.0).text("Music"));
        ui.add(egui::Slider::new(&mut game_volume.sfx_volume, 0.0..=1.0).text("SFX"));
    });
}