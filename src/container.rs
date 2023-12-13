use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::geometry::Collider;

pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_container);   
    }
}

#[derive(Component)]
pub struct Container;

fn spawn_container(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pillar_size = Vec2::new(25.0, 600.0);
    let container_width = 250.0;
    let side_pillar_y: f32 = -37.0;

    let mut create_mesh = |pos: Vec2, size: Vec2| {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        })
        .insert(Container)
        .insert(Collider::cuboid(size.x / 2.0, size.y / 2.0))
        .insert(Name::new("Container"));
    };

    create_mesh(Vec2::new(-container_width, side_pillar_y), pillar_size);
    create_mesh(Vec2::new(container_width, side_pillar_y), pillar_size);
    create_mesh(Vec2::new(0.0, -325.0), Vec2::new(container_width * 2.0, 25.0));
}