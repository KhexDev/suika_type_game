use bevy::{prelude::*, sprite::Anchor};

use crate::{fruits::FruitFusedEvent, utils::get_score_from_fruits};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_systems(Startup, create_text);
        app.add_systems(Update, (
            update_score,
            update_text,
        ));
    }
}

#[derive(Resource)]
struct Score(i32);
impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

#[derive(Component)]
struct ScoreText;

fn create_text(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_sections(vec![
            TextSection::new("Score: ", TextStyle {
                font_size: 32.0,
                ..default()
            }),
            TextSection::new("0", TextStyle {
                font_size: 32.0,
                ..default()
            }),
        ]),
        transform: Transform::from_xyz(-640.0, 360.0, 50.0),
        text_anchor: Anchor::TopLeft,
        ..default()
    })
    .insert(ScoreText);
}

fn update_score(
    mut score: ResMut<Score>,
    mut fruit_fused_ev: EventReader<FruitFusedEvent>,
) {
    for ev in fruit_fused_ev.read() {
        score.0 += get_score_from_fruits(ev.0);
    }
}

fn update_text(
    score: Res<Score>,
    mut text_q: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        let mut text = text_q.single_mut();
        text.sections[1].value = format!("{}", score.0);
    }

}