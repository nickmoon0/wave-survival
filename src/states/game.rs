use bevy::prelude::*;

use super::GameState;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .add_systems(
            Update,
            (handle_input, animate_sprites).chain().run_if(in_state(GameState::InGame)));
}

fn setup_game(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_sprite_sheet_cols: u32 = 8;
    let player_sprite_sheet_rows: u32 = 3;

    let texture = asset_server.load("player-sheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), 
        player_sprite_sheet_cols, 
        player_sprite_sheet_rows, 
        None, 
        None
    );
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    let player_animation_indices = AnimationIndices {
        row_first_idx: 1,
        row_last_idx: 7,

        side_row: 0,
        up_row: 1,
        down_row: 2,

        total_cols: player_sprite_sheet_cols as usize,
    };

    // Spawn player sprite
    commands.spawn((
        DespawnOnExit(GameState::InGame),
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas { 
                layout: texture_atlas_layout, 
                index: player_animation_indices.down_row * player_animation_indices.total_cols
            }
        ),
        Transform::from_scale(Vec3::splat(1.0)),
        player_animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerState { is_moving: false, direction: SpriteDirection::Down }
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut Transform)>
) {
    let (mut state, mut transform) = match query.single_mut() {
        Ok(state) => state,
        Err(e) => {
            println!("Failed to get character state. Err: {}", e);
            return;
        }
    };

    state.is_moving = false;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        state.is_moving = true;
        state.direction = SpriteDirection::Up;
        transform.translation.y = transform.translation.y + 1.0;
    } else if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        state.is_moving = true;
        state.direction = SpriteDirection::Down;
        transform.translation.y = transform.translation.y - 1.0;
    } else if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        state.is_moving = true;
        state.direction = SpriteDirection::Left;
        transform.translation.x = transform.translation.x - 1.0;
    } else if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        state.is_moving = true;
        state.direction = SpriteDirection::Right;
        transform.translation.x = transform.translation.x + 1.0;
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &PlayerState, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, player_state, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        sprite.flip_x = match player_state.direction {
            SpriteDirection::Left => true,
            _ => false
        };

        let (idle, start, end) = match player_state.direction {
            SpriteDirection::Up => (
                indices.up_row * indices.total_cols,
                indices.up_row * indices.total_cols + indices.row_first_idx,
                indices.up_row * indices.total_cols + indices.row_last_idx
            ),
            SpriteDirection::Down => (
                indices.down_row * indices.total_cols,
                indices.down_row * indices.total_cols + indices.row_first_idx,
                indices.down_row * indices.total_cols + indices.row_last_idx
            ),
            _ => (
                indices.side_row * indices.total_cols,
                indices.side_row * indices.total_cols + indices.row_first_idx,
                indices.side_row * indices.total_cols + indices.row_last_idx
            )
        };

        if player_state.is_moving {
            if timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index < start || atlas.index > end {
                    atlas.index = start;
                }

                atlas.index = if atlas.index == end {
                    start
                } else {
                    atlas.index + 1
                }
            }
        } else {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = idle;
            }
        }
    }
}

#[derive(Component)]
struct AnimationIndices {
    row_first_idx: usize,
    row_last_idx: usize,

    up_row: usize,
    down_row: usize,
    side_row: usize,

    total_cols: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct PlayerState {
    is_moving: bool,
    direction: SpriteDirection,
}

enum SpriteDirection {
    Up,
    Down,
    Left,
    Right
}