use bevy::prelude::*;

pub struct MousePosPlugin;

impl Plugin for MousePosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>();
        app.add_systems(Update, update_mouse_pos);
    }
}

#[derive(Resource)]
pub struct MousePos(pub Vec2);

impl Default for MousePos {
    fn default() -> Self {
        MousePos(Vec2::ZERO)
    }
}

fn update_mouse_pos(
    mut mouse_pos: ResMut<MousePos>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_pos.0 = world_position;
    }
}