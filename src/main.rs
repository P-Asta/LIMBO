use bevy::{prelude::*, ui::RelativeCursorPosition};
use rand::{thread_rng, Rng};

#[derive(Component)]
struct Key {
    position: usize,
    real: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, key_movement)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    for i in 0..8 {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    color: Color::Rgba {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                        alpha: 0.5,
                    },
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Key {
                position: i,
                real: false,
            });
    }
}

fn get_pos(id: usize) -> (f32, f32) {
    let n = 75.;
    [
        (n, n * 3.),
        (n, -n * 3.),
        (n, -n),
        (n, n),
        (-n, n),
        (-n, -n),
        (-n, n * 3.),
        (-n, -n * 3.),
    ][id]
}

fn key_movement(mut q: Query<(&mut Transform, &Key), With<Key>>) {
    for (mut transform, key) in q.iter_mut() {
        let pos = transform.translation;
        transform.translation +=
            (Vec3::new(get_pos(key.position).0, get_pos(key.position).1, 0.) - pos) / 10.
    }
}
