use bevy::prelude::*;

use super::GameState;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::InGame), setup_game);
}

fn setup_game(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    let player_animation_indices = AnimationIndices {
        idle_idx: 0,
        first_idx: 1,
        last_idx: 7,

        side_row: 0,
        up_row: 1,
        down_row: 2,
    };

    // Spawn player sprite
    commands.spawn((
        DespawnOnExit(GameState::InGame),
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas { 
                layout: texture_atlas_layout, 
                index: 16 // Idle down index
            }
        ),
        player_animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerState { is_moving: false, direction: SpriteDirection::Down }
    ));
}

#[derive(Component)]
struct AnimationIndices {
    idle_idx: usize,
    first_idx: usize,
    last_idx: usize,

    up_row: usize,
    down_row: usize,
    side_row: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct PlayerState {
    is_moving: bool,
    direction: SpriteDirection
}

enum SpriteDirection {
    Up,
    Down,
    Left,
    Right
}