use bevy::prelude::States;

pub mod main_menu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Menu {
    None,
    #[default]
    MainMenu,
}

