use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

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
        Player {},
    ));
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if key_input.pressed(KeyCode::Left) || key_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        } 
        if key_input.pressed(KeyCode::Right) || key_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        } 
        if key_input.pressed(KeyCode::Up) || key_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        } 
        if key_input.pressed(KeyCode::Down) || key_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * 100. * time.delta_seconds();
    }
}
