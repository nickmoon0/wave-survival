use bevy::prelude::*;

use super::Menu;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(Menu::MainMenu), setup_main_menu)
        .add_systems(Update, handle_input.run_if(in_state(Menu::MainMenu)));
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Menu::MainMenu),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![(
                Text::new("Press <space> to start."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                Node {
                    margin: UiRect::all(px(50)),
                    ..default()
                }
            )]
        )]
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_menu.set(Menu::None);
    }
}