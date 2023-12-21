use bevy::{prelude::*, sprite::MaterialMesh2dBundle, audio::{PlaybackMode, Volume, VolumeLevel}};
use bevy_rapier2d::{
    prelude::*,
    geometry::Collider,
};
use rand::prelude::*;
use once_cell::sync::Lazy;

use crate::{player::Player, container::Container, utils::get_fruits, volume::GameVolume};

pub struct FruitsPlugin;

impl Plugin for FruitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FruitFusedEvent>();
        app.add_event::<ChangedFruitEvent>();
        app.add_event::<CreateFruitEvent>();
        app.init_resource::<FruitList>();
        app.init_resource::<NextFruit>();
        app.register_type::<NextFruit>();
        app.add_systems(Update, (
            drop_fruit,
            check_fruits_placed,
            check_fruits_fusing,
            spawn_new_fruit_once_fused,
            clear_fruits,
            change_next_fruit,
            on_create_fruit,
            remove_fruits_out_of_reach,
            // check_fruits_flying,
            // check_loose_condition,
        ));
    }
}

#[derive(Component)]
struct Fruit;

#[derive(Component)]
struct FruitPlaced;

#[derive(Component)]
struct FruitFlying;

#[derive(Component, PartialEq, Clone, Copy, Debug, Reflect, Default)]
#[reflect(Component)]
pub enum FruitType {
    #[default]
    Cherry,
    Strawberry,
    Grapes,
    Dekopon,
    Persimon,
    Apple,
    Pear,
    Peach,
    Pineapple,
    Melon,
    Watermelon,
}

#[derive(Resource)]
struct FruitList(pub Vec<FruitType>);

impl Default for FruitList {
    fn default() -> Self {
        FruitList(vec![
            FruitType::Cherry,
            FruitType::Strawberry,
            FruitType::Grapes,
            FruitType::Dekopon,
            FruitType::Persimon,
            FruitType::Apple,
            FruitType::Pear,
            FruitType::Peach,
            FruitType::Pineapple,
            FruitType::Melon,
            FruitType::Watermelon,
        ])
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct NextFruit(pub FruitType);

impl Default for NextFruit {
    fn default() -> Self {
        NextFruit(FruitType::Strawberry)
    }
}

#[derive(Event)]
struct LoadNextFruit;

#[derive(Event)]
pub struct FruitFusedEvent(pub FruitType, pub Vec2);

#[derive(Event)]
struct ChangedFruitEvent;

#[derive(Event)]
struct CreateFruitEvent(pub FruitType, Option<Vec2>);

fn on_create_fruit(
    mut commands: Commands,
    q: Query<&Transform, With<Player>>,
    mut create_fruit_ev: EventReader<CreateFruitEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in create_fruit_ev.read() {
        let mut player_transform = q.single().clone().with_scale(Vec3::splat(100.0));
        let (ball_size, fruit_color ) = get_fruits(ev.0);

        if let Some(position) = ev.1 {
            player_transform.translation.x = position.x;
            player_transform.translation.y = position.y;
        }
    
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(ball_size))).into(),
            material: materials.add(ColorMaterial::from(fruit_color)).into(),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from(player_transform)))
        .insert(Collider::ball(ball_size))
        .insert(ColliderMassProperties::Mass(15.0 + (ev.0 as usize + 1) as f32 * 100.0))
        .insert(GravityScale(10.0))
        .insert(Friction::coefficient(0.75))
        .insert(Velocity {
            angvel: 0.0,
            linvel: Vec2::new(0.0, 0.0),
        })
        .insert(Fruit)
        .insert(ev.0)
        .insert(Name::new(format!("{:?}", ev.0)));
    }
}

fn drop_fruit(
    keys: Res<Input<KeyCode>>,
    btn: Res<Input<MouseButton>>,
    next_fruit: Res<NextFruit>,
    assets: Res<AssetServer>,
    time: Res<Time>,
    game_volume: Res<GameVolume>,
    mut change_fruit_ev: EventWriter<ChangedFruitEvent>,
    mut create_fruit_ev: EventWriter<CreateFruitEvent>,
    mut played_first: Local<bool>,
    mut commands: Commands,
    mut cooldown: Local<Timer>,
    mut setup: Local<bool>,
) {
    if !*setup {
        *cooldown = Timer::from_seconds(1.25, TimerMode::Once);
        *setup = true;
    }

    cooldown.tick(time.delta());

    if (keys.just_pressed(KeyCode::Space) || btn.just_pressed(MouseButton::Right)) && cooldown.finished() {
        let fruit_type: FruitType;
    
        if !*played_first {
            fruit_type = FruitType::Cherry;
            change_fruit_ev.send(ChangedFruitEvent);
            *played_first = true;
        } else {
            fruit_type = next_fruit.0;
            change_fruit_ev.send(ChangedFruitEvent);
        }

        commands.spawn(AudioBundle {
            source: assets.load("drop.mp3"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::Absolute(VolumeLevel::new(game_volume.sfx_volume)),
                ..default()
            },
            ..default()
        });

        create_fruit_ev.send(CreateFruitEvent(fruit_type, None));

        cooldown.reset();
    }
}

static mut RNG: Lazy<ThreadRng> = Lazy::new(|| thread_rng());

fn change_next_fruit(
    mut next_fruit: ResMut<NextFruit>,
    fruit_list: Res<FruitList>,
    mut change_fruit_ev: EventReader<ChangedFruitEvent>,
) {
    for _ in change_fruit_ev.read() {
        unsafe {
            let selected_type = RNG.gen_range(0..=4);
            next_fruit.0 = fruit_list.0[selected_type];    
        }
    }
}

fn check_fruits_placed(
    container_q: Query<Entity, With<Container>>,
    fruits_q: Query<Entity, (With<Fruit>, Without<FruitPlaced>)>,
    placed_fruits_q: Query<Entity, (With<Fruit>, With<FruitPlaced>)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for fruit_entity in fruits_q.iter() {
        for container_entity in container_q.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(fruit_entity, container_entity) {
                if contact_pair.has_any_active_contacts() {
                    // println!("colliding with fruit and container");
                    commands.entity(fruit_entity).insert(FruitPlaced);
                }
            }
        }
        for placed_fruit_entity in placed_fruits_q.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(fruit_entity, placed_fruit_entity) {
                if contact_pair.has_any_active_contacts() {
                    // println!("colliding with fruit and placed fruits");
                    commands.entity(fruit_entity).insert(FruitPlaced);
                }
            }
        }
    }
}

fn check_fruits_fusing(
    q: Query<(Entity, &FruitType, &Transform), (With<Fruit>, With<FruitPlaced>)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut fruits_fused_ev: EventWriter<FruitFusedEvent>,
) {
    let mut collided = false;
    for (first_entity, first_type, _) in q.iter() {
        for (second_entity, second_type, transform) in q.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(first_entity, second_entity) {
                if contact_pair.has_any_active_contacts() && first_type == second_type {
                    fruits_fused_ev.send(FruitFusedEvent(*first_type, Vec2::new(transform.translation.x, transform.translation.y)));
                    commands.entity(first_entity).despawn();
                    commands.entity(second_entity).despawn();
                    collided = true;
                    break;
                }
            }
        }
        if collided {
            break;
        }
    }
}

fn spawn_new_fruit_once_fused(
    fruit_list: Res<FruitList>,
    mut fruits_fused_ev: EventReader<FruitFusedEvent>,
    mut create_fruit_ev: EventWriter<CreateFruitEvent>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for ev in fruits_fused_ev.read() {
        let next_type = fruit_list.0[ev.0 as usize + 1];
        create_fruit_ev.send(CreateFruitEvent(next_type, Some(ev.1)));
        commands.spawn(AudioBundle {
            source: assets.load("fuse.mp3"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
            ..default()
        });
    }
}

fn remove_fruits_out_of_reach(
    q: Query<(Entity, &Transform), With<Fruit>>,
    mut commands: Commands,
) {
    for (entity, transform) in q.iter() {
        if transform.translation.y <= -800.0 {
            commands.entity(entity).despawn();
            // println!("out of reach removing fruit");
        }
    }
}

// MAY HAVE FIXED IT IDK
fn check_fruits_flying(
    grounded_fruit_q: Query<(Entity, &Velocity), (With<Fruit>, With<FruitPlaced>, Without<FruitFlying>)>,
    flying_fruit_q: Query<(Entity, &Velocity), (With<Fruit>, With<FruitPlaced>, With<FruitFlying>)>,
    mut commands: Commands,
) {
    let cond_vel = 100.0;

    for (entity, vel) in grounded_fruit_q.iter() {
        if vel.linvel.y.abs() >= cond_vel {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.insert(FruitFlying);
            }
        }
    }
    for (entity, vel) in flying_fruit_q.iter() {
        if vel.linvel.y.abs() <= cond_vel {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.remove::<FruitFlying>();
            }
        }
    }
}

fn check_loose_condition(
    placed_fruits_q: Query<&Transform, (With<Fruit>, With<FruitPlaced>, Without<FruitFlying>)>,
    fruits_q: Query<Entity, With<Fruit>>,
    mut next_fruit: ResMut<NextFruit>,
    mut commands: Commands,
) {
    for transform in placed_fruits_q.iter() {
        let offset = 75.0;
        if transform.translation.y >= 285.0 && transform.translation.y <= 285.0 + offset {
            println!("GAME OVER BIG NOOBS");
            for entity in fruits_q.iter() {
                commands.entity(entity).despawn();
                next_fruit.0 = FruitType::Cherry;
            }
        }   
    }
}

fn clear_fruits(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    q: Query<Entity, With<Fruit>>,
) {
    if keys.just_pressed(KeyCode::R) {
        for entity in q.iter() {
            commands.entity(entity).despawn();
        }
    }
}