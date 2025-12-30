mod shapes;
mod weapons;
mod entities;
mod utils;

use weapons::weapon_handler;
use entities::{Player, Enemy, enemy_handler};
use std::{ops::Add};
use raylib::{color::Color, prelude::*};
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Hello, World")
        .vsync()
        .build();
    let mut player = Player::new();
    let mut camera: Camera2D = Camera2D { 
        offset: Vector2 { x: WIDTH/2.0, y: HEIGHT/2.0}, 
        target: player.fields.position, 
        rotation: 0.0, 
        zoom: 1.2 };
    let camera_lookahead = 10.0;
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullet_cooldown: f32 = 0.0;
    let mut enemy_cooldown: f32 = 0.0;

    while !rl.window_should_close() {
        let mut input = Vector2::new(0.0, 0.0);
        if rl.is_key_down(KeyboardKey::KEY_W) { input.y -= 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_A) { input.x -= 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_S) { input.y += 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_D) { input.x += 1.0; }

        let velocity = input.normalized().scale_by(player.fields.speed * rl.get_frame_time());
        player.fields.position += velocity;

        camera.target = Vector2 { 
            x: lerp(camera.target.x, player.fields.position.x.add(velocity.x * camera_lookahead), 0.1),
            y: lerp(camera.target.y, player.fields.position.y.add(velocity.y * camera_lookahead), 0.1)
        };

        weapon_handler(&rl, &mut bullet_cooldown, &mut player); 
        enemy_handler(&rl, &mut enemies, &mut enemy_cooldown, &mut player);

        let direction_angle = player.fields.direction.y.atan2(player.fields.direction.x).to_degrees();
        
        let mut d = rl.begin_drawing(&thread);
        if player.damage.hitpoint > 0 { d.clear_background(Color::WHITE) } else { d.clear_background(Color::RED) }
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_text(
            format!("x: {:.1?}, y: {:.1?}, bullets: {}, direction: {}, {}, {:.1?} degrees", 
                velocity.x, 
                velocity.y, 
                player.bullets.len(), 
                player.fields.direction.x.round(), 
                player.fields.direction.y.round(), 
                direction_angle
            ).as_str(),
            12, HEIGHT as i32-40, 20, Color::BLACK);

        let mut d = d.begin_mode2D(camera);
        if !player.supposed_to_be_dead() {
            d.draw_circle_v(player.fields.position, player.fields.radius, player.fields.color);
            d.draw_text(format!("{}", player.damage.hitpoint).as_str(), player.fields.position.x as i32, player.fields.position.y as i32, 20, Color::RED);
            d.draw_text(format!("Enemies killed: {}", player.kill_count).as_str(), player.fields.position.x as i32 - 58, player.fields.position.y as i32 + 25, 15, Color::RED);
        } else { 
            d.draw_circle_v(player.fields.position, player.fields.radius, Color::WHITE);
            d.draw_text("YOU ARE SUPPOSED TO BE DEAD.", player.fields.position.x as i32 - 250, player.fields.position.y as i32 - 50, 30, Color::WHITE);
            d.draw_text(format!("Enemies killed: {}", player.kill_count).as_str(), player.fields.position.x as i32 - 58, player.fields.position.y as i32 + 25, 15, Color::WHITE);
        }

        
        if player.equipped == 0 {
            let (gun, _) = &player.weapons;
            d.draw_rectangle_pro(gun.fields.rect, gun.fields.origin, gun.fields.rotation, gun.fields.color);
        } else if player.equipped == 1 {
            let (_, sword) = &player.weapons;
            d.draw_rectangle_pro(sword.fields.rect, sword.fields.origin, sword.fields.rotation, sword.fields.color);
        }

        for bullet in &player.bullets { d.draw_circle_v(bullet.fields.position, bullet.fields.radius, bullet.fields.color); }
        for enemy in &enemies { 
            if !player.supposed_to_be_dead() {
                d.draw_circle_v(enemy.fields.position, enemy.fields.radius, enemy.fields.color);
                d.draw_text(format!("{}", enemy.damage.hitpoint).as_str(), enemy.fields.position.x as i32, enemy.fields.position.y as i32, 20, Color::BLACK);
            } else {
                d.draw_circle_v(enemy.fields.position, enemy.fields.radius, Color::BLACK);
                d.draw_text(format!("{}", enemy.damage.hitpoint).as_str(), enemy.fields.position.x as i32, enemy.fields.position.y as i32, 20, Color::RED);
            }
        }
        if !player.supposed_to_be_dead() {
            d.draw_rectangle_lines_ex(Rectangle {x: 0.0, y:0.0, width: WIDTH, height: HEIGHT}, 5.0, Color::BLACK);
        } else {
            d.draw_rectangle_lines_ex(Rectangle {x: 0.0, y:0.0, width: WIDTH, height: HEIGHT}, 5.0, Color::WHITE);
            d.draw_circle_gradient(player.fields.position.x as i32, player.fields.position.y as i32, 300.0, Color {r: 255, g: 255, b: 255, a:100}, Color {r: 255, g: 0, b: 0, a:0});
        }
        
    }
}