use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::{
    prelude::*,
    geometry::Collider,
};

use crate::mouse_pos::MousePos;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, move_player);
    }
}

const SPEED: f32 = 200.0;

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
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = q.get_single_mut() {
        // transform.translation.x = mouse_pos.0.x;
        if keys.pressed(KeyCode::Right) {
            transform.translation.x += SPEED * time.delta_seconds();
        } else if keys.pressed(KeyCode::Left) {
            transform.translation.x -= SPEED * time.delta_seconds();
        }
    }
}
