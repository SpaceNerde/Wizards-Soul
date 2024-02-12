use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CameraHandlerPlugin;

impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, camera_zoom);
    }
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width()/2., window.height()/2., 0.),
            ..default()
        },
        MainCamera,
    ));
}

fn camera_zoom(
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if let mut projection = camera_query.single_mut() {
        projection.scaling_mode = ScalingMode::WindowSize(4.0);
    }
}
