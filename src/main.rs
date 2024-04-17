use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.68, 0.87, 0.89);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // map
    commands.spawn(SpriteBundle {
        texture: asset_server.load("aliahan_town_house.png"),
        transform: Transform {
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..default()
    });

    // sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("hero/down1.png"),
        transform: Transform {
            translation: Vec3::new(-16.0, 0.0, 0.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..default()
    });
}
