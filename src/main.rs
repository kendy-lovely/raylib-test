#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod basic;
mod weapons;
mod entities;
mod utils;

use weapons::weapon_handler;
use entities::{Player, Level, enemy_handler, player_handler};
use std::{ops::Add};
use raylib::{color::Color, prelude::*};
use utils::Prompt;

use crate::utils::Cooldown;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("RAYLIB")
        .vsync()
        .build();
    let mut player = Player::new();
    let mut camera = Camera2D { 
        offset: Vector2 { x: WIDTH/2.0, y: HEIGHT/2.0}, 
        target: player.fields.position, 
        rotation: 0.0, 
        zoom: 1.2 };
    let camera_lookahead = 12.5;
    let mut level = Level {
        enemies: Vec::new(),
        enemy_cooldown: Cooldown {
            cooldown: 20.0,
            cooldown_value: 0.0
        },
        prompt: Prompt::new()
    };

    while !rl.window_should_close() {
        if level.prompt.appear { Prompt::select(&mut level, &mut rl, &mut player) }
        
        let mut input = Vector2::new(0.0, 0.0);
        if player.damage.damage_cooldown.cooldown_value < 20.0 {
            if rl.is_key_down(KeyboardKey::KEY_W) { input.y -= 1.0; }
            if rl.is_key_down(KeyboardKey::KEY_A) { input.x -= 1.0; }
            if rl.is_key_down(KeyboardKey::KEY_S) { input.y += 1.0; }
            if rl.is_key_down(KeyboardKey::KEY_D) { input.x += 1.0; }
        }

        let shake = player_handler(&rl, &mut player, &input, &mut level);
        weapon_handler(&rl, &mut player); 
        enemy_handler(&rl, &mut level, &mut player);

        camera.target = Vector2 { 
            x: lerp(camera.target.x, player.fields.position.x.add(player.fields.direction.x * camera_lookahead + shake as f32), camera_lookahead / 100.0),
            y: lerp(camera.target.y, player.fields.position.y.add(player.fields.direction.y * camera_lookahead + shake as f32), camera_lookahead / 100.0)
        };
        
        let mut d = rl.begin_drawing(&thread);
        if !player.supposed_to_be_dead() { d.clear_background(Color::WHITE) } else { d.clear_background(Color::RED) }

        d.draw_mode2D(camera, |mut d, _| {
            if !player.supposed_to_be_dead() {
                d.draw_circle_v(player.fields.position, player.fields.radius, player.fields.color.tint(Color { r: (player.damage.damage_cooldown.cooldown_value * 8.0).min(255.0).round() as u8, g: 0, b: 0, a: 255 }));
                d.draw_text(format!("{}", player.damage.hitpoint).as_str(), player.fields.position.x as i32, player.fields.position.y as i32, 20, Color::RED);
                d.draw_text(format!("Enemies killed: {}", player.kill_count).as_str(), player.fields.position.x as i32 - 58, player.fields.position.y as i32 + 25, 15, Color::RED);
            } else { 
                d.draw_circle_v(player.fields.position, player.fields.radius, Color::WHITE);
                d.draw_text("YOU ARE SUPPOSED TO BE DEAD.", player.fields.position.x as i32 - 250, player.fields.position.y as i32 - 50, 30, Color::WHITE);
                d.draw_text(format!("Enemies killed: {}", player.kill_count).as_str(), player.fields.position.x as i32 - 58, player.fields.position.y as i32 + 25, 15, Color::WHITE);
            }

            let (gun, sword) = &player.weapons;

            match player.equipped {
                0 => d.draw_rectangle_pro(gun.fields.rect, gun.fields.origin, gun.fields.rotation, gun.fields.color),
                1 => d.draw_rectangle_pro(sword.fields.rect, sword.fields.origin, sword.fields.rotation, sword.fields.color),
                _ => {}
            }

            for bullet in &gun.bullets { d.draw_circle_v(bullet.fields.position, bullet.fields.radius, bullet.fields.color); }
            for enemy in &level.enemies { 
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
            if level.prompt.appear {
                d.draw_text(format!("{}", level.prompt.text.0).as_str(), player.fields.position.x as i32-200, player.fields.position.y as i32-50, 18, Color::BLACK);
                d.draw_text(format!("{}", level.prompt.text.1).as_str(), player.fields.position.x as i32+100, player.fields.position.y as i32-50, 18, Color::BLACK);
            }
        });
        let (gun, sword) = &player.weapons;
        d.draw_text(format!("Gun({}) Cooldown: {:.1?}, Sword({}) Damage: {:1?}", gun.level, gun.reload.cooldown, sword.level, sword.damage).as_str(), 20, HEIGHT as i32-25, 20, Color::BLACK);
    }
}