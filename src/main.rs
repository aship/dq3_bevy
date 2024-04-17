use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.68, 0.87, 0.89);

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    move_cooldown: Timer,
}

#[derive(Resource, Default)]
struct Game {
    player: Player,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(FixedUpdate, move_sprite)
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    game.player.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);

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
    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("hero/down1.png"),
                transform: Transform {
                    translation: Vec3::new(-16.0, 0.0, 0.0),
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            })
            .id(),
    );
}

fn move_sprite(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut query: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;

        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction_y += 1.0;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction_y -= 1.0;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction_x += 1.0;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction_x -= 1.0;
            moved = true;
        }

        if moved {
            game.player.move_cooldown.reset();

            let mut transform = query.get_mut(game.player.entity.unwrap()).unwrap();

            let new_x = transform.translation.x + direction_x * 32.0;
            let new_y = transform.translation.y + direction_y * 32.0;

            transform.translation.x = new_x;
            transform.translation.y = new_y;
        }
    }
}
