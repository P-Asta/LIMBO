use rand::Rng;
use system_shutdown::sleep as lmao;

use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, mouse::MouseButton, Input},
    math::Vec3,
    render::color::Color,
    sprite::Sprite,
    time::{Time, Timer},
    transform::components::Transform,
    window::Window,
};

#[derive(Component)]
pub struct Key {
    pub position: isize,
    pub real: bool,
    pub timer: Timer,
}

static mut N: usize = 0;

fn get_pos(id: isize) -> (f32, f32) {
    let mut id: isize = id % 8;
    if id < 0 {
        id = id % 8 + 8;
    }
    let n = 75.;
    [
        (-n, n * 3.),
        (n, n * 3.),
        (n, n),
        (n, -n),
        (n, -n * 3.),
        (-n, -n * 3.),
        (-n, -n),
        (-n, n),
    ][id as usize]
}

fn get_color(id: isize, n: usize) -> (f32, f32, f32) {
    let mut id: isize = id % 8;
    if id < 0 {
        id = id % 8 + 8;
    }

    if n >= 360 {
        [
            (3., 0.5, 0.5),
            (1.5, 2.5, 0.5),
            (0.5, 3., 0.5),
            (0.5, 1.5, 2.5),
            (0.5, 0.5, 3.),
            (0.5, 0.5, 1.5),
            (0.5, 0.5, 0.5),
            (1., 1., 1.),
        ][id as usize]
    } else {
        (3., 0.5, 0.5)
    }
}

pub fn movement(
    mut q: Query<(&mut Transform, &mut Key, &mut Sprite), With<Key>>,
    time: Res<Time>,
    k: Res<Input<KeyCode>>,
) {
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
            n = rng.gen_range(-6..=7);
        }
    }
    for (mut transform, mut key, mut sprite) in q.iter_mut() {
        let pos = transform.translation;
        key.timer.tick(time.delta());

        if unsafe { (N == 32 || N == 49 || N == 66) && key.position == REAL } {
            key.real = true;
            sprite.color = Color::Rgba {
                red: 0.5,
                green: 3.,
                blue: 0.5,
                alpha: 1.,
            };
            unsafe {
                if N <= 360 {
                    N += 1
                }
            }
        }
        if key.timer.finished() {
            key.position += n;
            unsafe {
                N += 1;
            }
        }
        if unsafe { N >= 360 } {
            unsafe {
                N += 1;
                transform.translation += (Vec3::new(
                    f32::sin(N as f32 / 1000. + key.position as f32 / 1.25) * 300.,
                    f32::cos(N as f32 / 1000. + key.position as f32 / 1.25) * 200.,
                    0.,
                ) - pos)
                    / 30.;
            }
        } else {
            transform.translation +=
                (Vec3::new(get_pos(key.position).0, get_pos(key.position).1, 0.) - pos) / 15.;
        }
        // println!("{}", k.pressed(KeyCode::Key5));
        if k.pressed(KeyCode::Key5)
            && k.pressed(KeyCode::Key2)
            && k.pressed(KeyCode::Key3)
            && key.real
        {
            sprite.color = Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 1.,
            }
        }
        sprite.color = Color::Rgba {
            red: sprite.color.as_rgba().r()
                + (get_color(key.position, unsafe { N }).0 - sprite.color.as_rgba().r()) / 20.,
            green: sprite.color.as_rgba().g()
                + (get_color(key.position, unsafe { N }).1 - sprite.color.as_rgba().g()) / 20.,
            blue: sprite.color.as_rgba().b()
                + (get_color(key.position, unsafe { N }).2 - sprite.color.as_rgba().b()) / 20.,
            alpha: sprite.color.as_rgba().a() + (1. - sprite.color.as_rgba().a()) / 20.,
        };
    }
}

pub fn click(
    mut q: Query<(&mut Transform, &mut Key), With<Key>>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();
    let Some(mut cursor_position) = window.cursor_position() else {
        return;
    };
    cursor_position[0] -= width / 2.;
    cursor_position[1] -= height / 2.;
    cursor_position[1] = -cursor_position[1];
    // windows.sc
    for (transform, key) in q.iter() {
        if (transform.translation.x <= cursor_position.x + 50.
            && transform.translation.x >= cursor_position.x - 50.)
            && (transform.translation.y <= cursor_position.y + 50.
                && transform.translation.y >= cursor_position.y - 50.)
            && mouse.just_pressed(MouseButton::Left)
        {
            if unsafe { N >= 360 } {
                if key.real {
                    std::process::exit(0x7777777);
                } else {
                    match lmao() {
                        Ok(_) => {
                            println!("it's UNREAL LMAO");
                        }
                        Err(_) => {
                            println!("it's UNREAL & why dont work sleep :(");
                        }
                    }
                }
            }
        }
    }
    // println!("{}", cursor_position)
}
