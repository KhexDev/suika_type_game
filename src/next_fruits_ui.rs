use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{fruits::NextFruit, utils::get_fruits};

pub struct NextFruitUIPlugin;

impl Plugin for NextFruitUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_next_fruit_ui);
        app.add_systems(Update, update_next_fruit_ui);
    }
}

#[derive(Component)]
struct NextFruitUI;

fn create_next_fruit_ui(
    next_fruit: Res<NextFruit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let (ball_size, fruit_color) = get_fruits(next_fruit.0);
    println!("next fruit ui: {:?}", next_fruit.0);

    commands.spawn(ColorMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(ball_size * 100.0))).into(),
        material: materials.add(ColorMaterial::from(fruit_color)).into(),
        transform: Transform::from_xyz(400.0, 300.0, 0.0),
        ..default()
    })
    .insert(NextFruitUI)
    .insert(Name::new("Next Fruit UI"));
}

fn update_next_fruit_ui(
    next_fruit: Res<NextFruit>,
    q: Query<(&Mesh2dHandle, &Handle<ColorMaterial>), With<NextFruitUI>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    if !next_fruit.is_changed() {
        return;
    }

    let (mesh_handle, material_handle) = q.single();
    let  mesh_option = meshes.get_mut(&mesh_handle.0);
    let material_option = materials.get_mut(material_handle.id());
    let (ball_size, fruit_color) = get_fruits(next_fruit.0);

    if let Some(mut _mesh) = mesh_option {
        *_mesh = Mesh::from(shape::Circle::new(ball_size * 100.0));
    } else {
        println!("couldnt change mesh");
    }

    if let Some(mut _material) = material_option {
        _material.color = fruit_color;
    } else {
        println!("couldnt change material");
    }
}