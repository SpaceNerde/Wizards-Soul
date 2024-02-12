mod player;
mod camera_handler;
mod magic;
mod dungeon_generator;

use crate::player::PlayerPlugin;
use crate::camera_handler::CameraHandlerPlugin;
use crate::magic::MagicPlugin;
use crate::dungeon_generator::DungeonGeneratorPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Game Logic
            PlayerPlugin,
            CameraHandlerPlugin,
            MagicPlugin,
            DungeonGeneratorPlugin,
        ));
    }
}
