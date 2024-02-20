use bevy::prelude::*;
use crate::magic::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, (player_movement, player_animation_handler, player_attack));
    }
}

#[derive(Component)]
struct Player {
    direction: Vec3,
    player_animation: PlayerAnimation,
}

enum PlayerAnimation {
    Walking
}

// Basic Starting Player setup 
fn player_setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    let texture = asset_server.load("sprites/character_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(16., 16.), 2, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle { 
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Player {
            direction: Vec3::ZERO,
            player_animation: PlayerAnimation::Walking,
        },
    ));
}

fn player_animation_handler(
    mut player_query: Query<(
        &mut Player,
        &mut TextureAtlasSprite
    )>
) {
    for (mut player, mut sprite) in &mut player_query {
        match player.player_animation {
            PlayerAnimation::Walking => {
                match player.direction {
                    _ => {},
                }
            },
        }
    }
}

// Handeling Player Movement Up Down Left Right
fn player_movement(
    mut animation_query: Query<&mut TextureAtlasSprite>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut sprite) = animation_query.get_single_mut() {
        if let Ok(mut transform) = query.get_single_mut() {
            let mut direction = Vec3::ZERO;

            if key_input.pressed(KeyCode::Left) || key_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1., 0., 0.);
                sprite.index = 3;
            } 
            if key_input.pressed(KeyCode::Right) || key_input.pressed(KeyCode::D) {
                direction += Vec3::new(1., 0., 0.);
                sprite.index = 1;
            } 
            if key_input.pressed(KeyCode::Up) || key_input.pressed(KeyCode::W) {
                direction += Vec3::new(0., 1., 0.);
                sprite.index = 7;
            } 
            if key_input.pressed(KeyCode::Down) || key_input.pressed(KeyCode::S) {
                direction += Vec3::new(0., -1., 0.);
                sprite.index = 5;
            }

            if direction.length() > 0. {
                direction = direction.normalize();
            }

            transform.translation += direction * 100. * time.delta_seconds();
        }
    }
}

// Handels all kinds of attack event? (means it does not include interact)
fn player_attack(
    query: Query<&Transform, With<Player>>,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    match query.get_single() {
        Ok(transform) => {
            if mouse_input.pressed(MouseButton::Left) {
            }
        }
        _ => {}
    }
}
