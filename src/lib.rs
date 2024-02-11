mod player;

use crate::player::PlayerPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Game Logic
            PlayerPlugin,
        ));
    }
}
