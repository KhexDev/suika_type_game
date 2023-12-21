use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::mouse_pos::MousePos;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, move_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_xyz(0.0, 300.0, 0.0).with_scale(Vec3::splat(50.0)),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..default()
    })
    .insert(Player)
    .insert(Name::new("Player"));
}

fn move_player(
    mut q: Query<&mut Transform, With<Player>>,
    mouse_pos: Res<MousePos>,
) {
    let lerp = |a, b, t| -> f32 {
        a + (b - a) * t
    };

    let range = 210.0;

    if let Ok(mut transform) = q.get_single_mut() {
        transform.translation.x = lerp(f32::clamp(mouse_pos.0.x, -range, range), transform.translation.x, 0.75);
    }
}
