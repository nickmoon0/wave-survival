use bevy::prelude::*;
use menus::{main_menu, Menu};
use states::{GameState, game};

mod menus;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((main_menu::plugin, game::plugin))
        .init_state::<Menu>()
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}