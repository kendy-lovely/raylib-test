use std::ops::Add;
use rand::{Rng, rngs::ThreadRng};
use raylib::{color::Color, prelude::*};
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

#[derive(Copy, Clone)] 
struct Ball {
    position: Vector2,
    direction: Vector2,
    speed: f32,
    radius: f32,
    color: Color
}

#[derive(Copy, Clone)] 
struct Rectangle {
    rect: ffi::Rectangle,
    origin: ffi::Vector2,
    rotation: f32,
    color: Color
}

struct Sword {
    fields: Rectangle
}

struct Gun {
    fields: Rectangle

}

#[derive(Copy, Clone)] 
struct Bullet {
    fields: Ball,
    duration: f32
}
struct Player { 
    fields: Ball,
    bullets: Vec<Bullet>,
    weapons: (Gun, Sword),
    equipped: usize
}
struct Enemy { fields: Ball }

fn round_to_nearest(x: f32, a: f32, b: f32) -> f32 {
    let (min_val, max_val) = (a.min(b), a.max(b));
    let diff_min = (x - min_val).abs();
    let diff_max = (x - max_val).abs();
    if x > min_val && x < max_val {
        if diff_min > diff_max { b } else if diff_min < diff_max { a } else { a }
    } else { x }
}

fn weapon_handler(rl: &RaylibHandle, cooldown: &mut f32, player: &mut Player) {
    let direction = &mut player.fields.direction;
    let bullets = &mut player.bullets;
    if rl.is_key_down(KeyboardKey::KEY_UP) { 
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_RIGHT) 
        { direction.y = (direction.y - 1.0).max(-1.0) } else { *direction = Vector2 { x: (0.0), y: (-1.0) } }
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) { 
        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_RIGHT)
        { direction.x = (direction.x - 1.0).max(-1.0) } else { *direction = Vector2 { x: (-1.0), y: (0.0) } }
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) { 
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_RIGHT)
        { direction.y = (direction.y + 1.0).min(1.0) } else { *direction = Vector2 { x: (0.0), y: (1.0) } }
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) { 
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_UP)
        { direction.x = (direction.x + 1.0).min(1.0) } else { *direction = Vector2 { x: (1.0), y: (0.0) } }
    }
        
    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_UP) {
        if *cooldown <= 0.0 && player.equipped == 0 {
            let bullet = Bullet {
                fields: Ball {
                    position: Vector2 { x: player.weapons.0.fields.rect.x, y: player.weapons.0.fields.rect.y },
                    direction: Vector2::from(*direction),
                    speed: 500.0,
                    radius: 5.0,
                    color: Color::GOLD
                },
                duration: 0.0
            };
            bullets.push(bullet);
            *cooldown = 2.;
        }
    }
    *cooldown = (*cooldown - 10.0 * rl.get_frame_time()).max(0.0);
    bullets.retain_mut(|bullet| bullet.duration < 15.0);
    for bullet in bullets {
        bullet.duration += 5.0 * rl.get_frame_time();
        let velocity = Vector2::scale_by(&bullet.fields.direction.normalized(), bullet.fields.speed * rl.get_frame_time());
        bullet.fields.position = Vector2::add(bullet.fields.position, velocity);
    }
}

fn enemy_handler(rl: &RaylibHandle, enemies: &mut Vec<Enemy>, cooldown: &mut f32, player: &mut Player, rng: &mut ThreadRng) {
    let player_pos = &mut player.fields.position;
    let bullets = &mut player.bullets;
    let sword_pos = &mut Vector2 { x: player.weapons.1.fields.rect.x, y: player.weapons.1.fields.rect.y };
    if *cooldown <= 0.0 {
        let enemy = Enemy {
            fields: Ball {
                position: Vector2::new(
                    round_to_nearest(rng.random::<f32>()*(WIDTH+400.0)-200.0, 0.0, WIDTH), 
                    round_to_nearest(rng.random::<f32>()*(HEIGHT-400.0)+200.0, HEIGHT, 0.0)
                ),
                direction: Vector2::new(0.0, 0.0),
                speed: 125.0,
                radius: rng.random_range(18.0..26.0),
                color: Color::RED
            }
        };
        enemies.push(enemy);
        *cooldown = 10.;
    }
    *cooldown = (*cooldown - 10.0 * rl.get_frame_time()).max(0.0);
    enemies.retain_mut(|enemy| { 
        !(bullets
            .iter()
            .any(|bullet| enemy.fields.position.distance_to(bullet.fields.position) < enemy.fields.radius) 
            && player.equipped == 0)
        &&
        !(sword_pos.distance_to(enemy.fields.position) < enemy.fields.radius * 2.0 && player.equipped == 1)
    });
    for enemy in enemies {
        let angle_dir = enemy.fields.position.angle_to(*player_pos);
        enemy.fields.direction = Vector2 { x: (angle_dir.cos()), y: (angle_dir.sin()) };
        let velocity = Vector2::scale_by(&enemy.fields.direction.normalized(), enemy.fields.speed * rl.get_frame_time());
        enemy.fields.position = Vector2::add(enemy.fields.position, velocity);
    }
}

fn main() {
    let mut rng = rand::rng();
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Hello, World")
        .vsync()
        .build();
    let mut player = Player {
        fields: Ball {
            position: Vector2::new(WIDTH/2.0, HEIGHT/2.0),
            direction: Vector2::new(0.0, 0.0),
            speed: 200.0,
            radius: 24.0,
            color: Color::BLACK
        },
        bullets: Vec::new(),
        weapons: (Gun {
                fields: Rectangle { 
                    rect: ffi::Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 20.0 }, 
                    origin: ffi::Vector2 {x: 5.0, y: 10.0}, 
                    rotation: 0.0, 
                    color: Color::GRAY 
                }
            }, Sword {
                fields: Rectangle { 
                    rect: ffi::Rectangle { x: 0.0, y: 0.0, width: 20.0, height: 80.0 }, 
                    origin: ffi::Vector2 {x: 10.0, y: 40.0}, 
                    rotation: 0.0, 
                    color: Color::SILVER 
                }
            },
        ),
        equipped: 0
    };
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullet_cooldown: f32 = 0.0;
    let mut enemy_cooldown: f32 = 0.0;

    while !rl.window_should_close() {
        let mut input = Vector2::new(0.0, 0.0);
        if rl.is_key_down(KeyboardKey::KEY_W) { input.y -= 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_A) { input.x -= 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_S) { input.y += 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_D) { input.x += 1.0; }
        let velocity = Vector2::scale_by(&input.normalized(), player.fields.speed * rl.get_frame_time());
        player.fields.position = Vector2::add(player.fields.position, velocity);

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if player.equipped == 0 {
                player.equipped = 1
            } else { player.equipped = 0 }
        }

        weapon_handler(&rl, &mut bullet_cooldown, &mut player);
        enemy_handler(&rl, &mut enemies, &mut enemy_cooldown, &mut player, &mut rng);

        let (gun, sword) = (&mut player.weapons.0, &mut player.weapons.1);
        let direction_angle = player.fields.direction.y.atan2(player.fields.direction.x);

        gun.fields.rect.x = lerp(gun.fields.rect.x, player.fields.position.x + 30.0 * direction_angle.cos(), 0.5);
        gun.fields.rect.y = lerp(gun.fields.rect.y, player.fields.position.y + 30.0 * direction_angle.sin(), 0.5);
        gun.fields.rotation = direction_angle.to_degrees().add(90.0);

        sword.fields.rect.x = lerp(sword.fields.rect.x, player.fields.position.x + 60.0 * direction_angle.cos(), 0.5);
        sword.fields.rect.y = lerp(sword.fields.rect.y, player.fields.position.y + 60.0 * direction_angle.sin(), 0.5);
        sword.fields.rotation = direction_angle.to_degrees().add(90.0);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_text(format!("x: {:.1?}, y: {:.1?}, bullets: {}, direction: {}, {}, {:.1?} degrees", velocity.x, velocity.y, player.bullets.len(), player.fields.direction.x, player.fields.direction.y, direction_angle.to_degrees()).as_str(), 12, HEIGHT as i32-40, 20, Color::BLACK);
        d.draw_circle_v(player.fields.position, player.fields.radius, player.fields.color);
        if player.equipped == 0 {
            d.draw_rectangle_pro(gun.fields.rect, gun.fields.origin, gun.fields.rotation, gun.fields.color);
        } else { 
            d.draw_rectangle_pro(sword.fields.rect, sword.fields.origin, sword.fields.rotation, sword.fields.color);
        }
        for bullet in &player.bullets { d.draw_circle_v(bullet.fields.position, bullet.fields.radius, bullet.fields.color); }
        for enemy in &enemies { d.draw_circle_v(enemy.fields.position, enemy.fields.radius, enemy.fields.color); }
    }
}