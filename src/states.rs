use bevy::prelude::States;

pub mod game;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    InMenu,
    InGame,
}