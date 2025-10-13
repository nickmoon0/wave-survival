use bevy::prelude::*;
use menus::{main_menu, Menu};

mod menus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(main_menu::plugin)
        .init_state::<Menu>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}