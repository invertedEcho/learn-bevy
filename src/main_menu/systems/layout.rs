use bevy::prelude::*;

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::NORMAL_BUTTON_COLOR,
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(Node {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageNode {
                        image: asset_server.load("sprites/ball_blue_large.png"),
                        ..default()
                    });
                    parent.spawn(Text::new("Bevy Ball Game"));
                    parent.spawn(ImageNode {
                        image: asset_server.load("sprites/ball_red_large.png"),
                        ..default()
                    });
                });
            // Play Button
            parent
                .spawn((
                    Node {
                        width: Val::Px(180.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button {},
                    PlayButton {},
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Play"));
                });
            // Quit
            parent
                .spawn((
                    Node {
                        width: Val::Px(180.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button {},
                    QuitButton {},
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Quit"));
                });
        })
        .id();
    main_menu_entity
}
