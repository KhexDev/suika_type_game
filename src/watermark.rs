use bevy::{prelude::*, sprite::Anchor};

pub struct WatermarkPlugin;

impl Plugin for WatermarkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Text2dBundle {
                text: Text::from_section("Goofy Suika Game v1.0.0", TextStyle {
                    font_size: 32.0,
                    ..default()
                }),
                text_anchor: Anchor::BottomRight,
                transform: Transform::from_xyz(640.0, -360.0, 50.0),
                ..default()
            });
        });
    }
}

