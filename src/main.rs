use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
    ui::RelativeCursorPosition,
};
use rand::{thread_rng, Rng};

#[derive(Component)]
struct Key {
    position: isize,
    real: bool,
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, key_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("bgm.wav"),
        settings: PlaybackSettings {
            volume: Volume::Absolute(VolumeLevel::new(0.5)),
            ..Default::default()
        },
        ..Default::default()
    });
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
                        alpha: 1.,
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
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            });
    }
}

fn get_pos(id: isize) -> (f32, f32) {
    let mut id: isize = id % 8;
    if id < 0 {
        id = id % 8 + 8;
    }
    let n = 75.;
    [
        (n, n * 3.),
        (-n, n * 3.),
        (n, n),
        (-n, n),
        (n, -n),
        (-n, -n),
        (n, -n * 3.),
        (-n, -n * 3.),
    ][id as usize]
}

fn key_movement(mut q: Query<(&mut Transform, &mut Key, &mut Sprite), With<Key>>, time: Res<Time>) {
    static mut N: usize = 0;
    static mut REAL: isize = 0;

    unsafe {
        if N == 32 {
            let mut rng = rand::thread_rng();
            REAL = rng.gen_range(0..=7);
        }
    }

    let mut n = 0;
    if unsafe { N >= 98 && N <= 352 } {
        let mut rng = rand::thread_rng();
        while n == 0 {
            n = rng.gen_range(-7..=7);
        }
    }
    for (mut transform, mut key, mut sprite) in q.iter_mut() {
        let pos = transform.translation;
        key.timer.tick(time.delta());

        if unsafe { (N == 32 || N == 49 || N == 66) && key.position == REAL } {
            key.real = true;
            sprite.color = Color::Rgba {
                red: 0.5,
                green: 1.,
                blue: 0.5,
                alpha: 1.,
            };
            unsafe { N += 1 }
        }
        if key.timer.finished() {
            key.position += n;
            unsafe {
                N += 1;
            }
        }
        transform.translation +=
            (Vec3::new(get_pos(key.position).0, get_pos(key.position).1, 0.) - pos) / 10.;
        sprite.color = Color::Rgba {
            red: sprite.color.as_rgba().r() + (0.5 - sprite.color.as_rgba().r()) / 20.,
            green: sprite.color.as_rgba().g() + (0.5 - sprite.color.as_rgba().g()) / 20.,
            blue: sprite.color.as_rgba().b() + (0.5 - sprite.color.as_rgba().b()) / 20.,
            alpha: sprite.color.as_rgba().a() + (1. - sprite.color.as_rgba().a()) / 20.,
        };
    }
}
