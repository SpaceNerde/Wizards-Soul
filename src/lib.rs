mod player;
mod camera_handler;

use crate::player::PlayerPlugin;
use crate::camera_handler::CameraHandlerPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Game Logic
            PlayerPlugin,
            CameraHandlerPlugin,
        ));
    }
}
