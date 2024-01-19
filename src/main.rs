use std::io::Cursor;

use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
    window::PrimaryWindow,
    winit::WinitWindows,
};
use winit::window::Icon;
mod key;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, set_window_icon)
        .add_systems(Startup, setup)
        .add_systems(Update, key::movement)
        .add_systems(Update, key::click)
        .run();
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../build/icon_1024x1024.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
        primary.set_title("LIMBO");
    };
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
                        red: 1.,
                        green: 1.,
                        blue: 1.,
                        alpha: 1.,
                    },
                    ..Default::default()
                },
                texture: asset_server.load("key.png"),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(key::Key {
                position: i,
                real: false,
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            });
    }
}
