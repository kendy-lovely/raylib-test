use crate::shapes::{RectanglePro, BallEnt};
use crate::entities::{Player};
use raylib::{color::Color, prelude::*};
use std::{ops::Add};
use crate::{WIDTH, HEIGHT};
pub struct Sword {
    pub fields: RectanglePro,
    pub offset: Vector2,
    pub is_swinging: bool,
    pub swing_progress: f32
}

impl Sword {
    pub fn new() -> Self {
        Self {
            fields: RectanglePro { 
                rect: ffi::Rectangle { x: 0.0, y: 0.0, width: 80.0, height: 20.0 }, 
                origin: ffi::Vector2 {x: 0.0, y: 0.0}, 
                rotation: 0.0, 
                color: Color::SILVER 
            },
            offset: Vector2 { x: 35.0, y: 0.0 },
            is_swinging: false,
            swing_progress: 75.0
        }
    }
    pub fn swing(&mut self) {
        self.is_swinging = true;
    }
}

pub struct Gun {
    pub fields: RectanglePro,
    pub offset: Vector2
}

impl Gun {
    pub fn new() -> Self {
        Self {
            fields: RectanglePro { 
                rect: ffi::Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 20.0 }, 
                origin: ffi::Vector2 {x: 5.0, y: 10.0}, 
                rotation: 0.0, 
                color: Color::GRAY 
            },
            offset: Vector2 { x: 20.0, y: 20.0 }
        }
    }
}

#[derive(Copy, Clone, PartialEq)] 
pub struct Bullet {
    pub fields: BallEnt,
    pub bounces: u8,
    pub hit_enemy: bool
}

impl Bullet {
    pub fn new(player: &Player) -> Self {
        Self {
            fields: BallEnt {
                position: Vector2 { x: player.weapons.0.fields.rect.x, y: player.weapons.0.fields.rect.y },
                direction: Vector2::from(player.fields.direction).normalized(),
                speed: 500.0,
                radius: 5.0,
                color: Color::GOLD
            },
            bounces: 0,
            hit_enemy: false
        }
    }
}

pub fn weapon_handler(rl: &RaylibHandle, cooldown: &mut f32, player: &mut Player) {
    let direction = &mut player.fields.direction;
    let direction_angle = direction.y.atan2(direction.x);
    let (gun, sword) = (&mut player.weapons.0, &mut player.weapons.1);
    let bullet_cooldown: f32 = 10.0;
    let rotation_smoothing = 0.35;

    const MAX_BOUNCES: u8 = 1;
    
    if rl.is_key_down(KeyboardKey::KEY_UP) { 
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_RIGHT) { 
            direction.y = lerp(direction.y, (direction.y - 1.0).max(-1.0), rotation_smoothing) 
        } else { 
            *direction = Vector2 { 
                x: lerp(direction.x, 0.0, rotation_smoothing), 
                y: lerp(direction.y, -1.0, rotation_smoothing) }}}
    if rl.is_key_down(KeyboardKey::KEY_LEFT) { 
        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_RIGHT) { 
            direction.x = lerp(direction.x, (direction.x - 1.0).max(-1.0), rotation_smoothing) 
        } else { 
            *direction = Vector2 { 
                x: lerp(direction.x, -1.0, rotation_smoothing), 
                y: lerp(direction.y, 0.0, rotation_smoothing) }}}
    if rl.is_key_down(KeyboardKey::KEY_DOWN) { 
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_RIGHT) { 
            direction.y = lerp(direction.y, (direction.y + 1.0).min(1.0), rotation_smoothing) 
        } else { 
            *direction = Vector2 { 
                x: lerp(direction.x, 0.0, rotation_smoothing), 
                y: lerp(direction.y, 1.0, rotation_smoothing) }}}
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) { 
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_UP) { 
            direction.x = lerp(direction.x, (direction.x + 1.0).min(1.0), rotation_smoothing) 
        } else { 
            *direction = Vector2 { 
                x: lerp(direction.x, 1.0, rotation_smoothing), 
                y: lerp(direction.y, 0.0, rotation_smoothing) }}}
    
    if rl.is_key_pressed(KeyboardKey::KEY_C) {
        if player.equipped + 1 > 1 { player.equipped = 0 } else { player.equipped += 1 }
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) { sword.swing() }

    let gun_offset = Vector2 {
        x: gun.offset.x * direction_angle.cos() - gun.offset.y * direction_angle.sin(), 
        y: gun.offset.x * direction_angle.sin() + gun.offset.y * direction_angle.cos()
    };
    
    let sword_offset = Vector2 { 
        x: sword.offset.x * direction_angle.add((-PI/2.0) as f32 + sword.swing_progress.to_radians()).cos() - sword.offset.y * direction_angle.add((-PI/2.0) as f32 + sword.swing_progress.to_radians()).sin(), 
        y: sword.offset.x * direction_angle.add((-PI/2.0) as f32 + sword.swing_progress.to_radians()).sin() + sword.offset.y * direction_angle.add((-PI/2.0) as f32 + sword.swing_progress.to_radians()).cos()
    };

    gun.fields.rect.x = player.fields.position.x + gun_offset.x;
    gun.fields.rect.y = player.fields.position.y + gun_offset.y;
    gun.fields.rotation = lerp(gun.fields.rotation, direction_angle.to_degrees().add(90.0), 0.5);

    
    if sword.swing_progress <= 0.0 { sword.is_swinging = false }
    if sword.is_swinging { sword.swing_progress = lerp(sword.swing_progress, -1.0, 0.25) } 
    else if sword.swing_progress != 75.0 { sword.swing_progress = lerp(sword.swing_progress, 75.0, 0.5) }

    sword.fields.rect = ffi::Rectangle { 
        x: player.fields.position.x + sword_offset.x,
        y: player.fields.position.y + sword_offset.y,
        width: sword.fields.rect.width,
        height: sword.fields.rect.height
    };
    sword.fields.rotation = lerp(sword.fields.rotation, direction_angle.add((-PI/4.0) as f32).to_degrees(), 0.5).add(sword.swing_progress);

    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_UP) {
        if *cooldown <= 0.0 && player.equipped == 0 {
            let bullet = Bullet::new(player);
            player.bullets.push(bullet);
            if player.damage.hitpoint != 0 { *cooldown = bullet_cooldown } else { *cooldown = bullet_cooldown/5.0 }
        }
    }
    *cooldown = (*cooldown - 10.0 * rl.get_frame_time()).max(0.0);
    player.bullets.retain_mut(|bullet| {
        if bullet.fields.position.x <= 0.0 || bullet.fields.position.x >= WIDTH { bullet.fields.direction.x = -bullet.fields.direction.x; bullet.bounces += 1 }
        if bullet.fields.position.y <= 0.0 || bullet.fields.position.y >= HEIGHT { bullet.fields.direction.y = -bullet.fields.direction.y; bullet.bounces += 1 }
        let velocity = Vector2::scale_by(&bullet.fields.direction, bullet.fields.speed * rl.get_frame_time());
        bullet.fields.position = Vector2::add(bullet.fields.position, velocity);
        bullet.bounces <= MAX_BOUNCES && bullet.hit_enemy == false
    });
}