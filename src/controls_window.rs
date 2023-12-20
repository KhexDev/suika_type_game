use bevy::prelude::*;
use bevy_inspector_egui::{egui, bevy_egui::EguiContexts};

pub struct ControlWindowPlugin;

impl Plugin for ControlWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_window);
    }
}

fn draw_window(mut contexts: EguiContexts) {
    egui::Window::new("Controles").show(contexts.ctx_mut(), |ui| {
        ui.separator();
        ui.label("Click droite: pour poser un fruit");
        ui.label("Touche R: pour delete tous les fruits");
        ui.separator();
        ui.label("Le circle en haut a droite indique le fruit actuel que tu va poser");
        ui.label("Voilà joue bien zebi");
        ui.label("PS: le jeu il est un peu beuger niveau physique sinon ca va");
        ui.label("-Jovani");
    });
}