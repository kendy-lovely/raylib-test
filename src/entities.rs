use crate::shapes::BallEnt;
use crate::weapons::{Gun, Sword, Bullet};
use crate::utils::round_to_nearest;
use rand::rng;
use raylib::{color::Color, prelude::*};
use rand::{Rng, rngs::ThreadRng};
use std::{ops::Add};
use crate::{WIDTH, HEIGHT};

pub struct DamageSystem {
    pub hitpoint: u8,
    pub damage_cooldown: f32,
    pub cooldown_value: f32
}
pub struct Player { 
    pub fields: BallEnt,
    pub bullets: Vec<Bullet>,
    pub weapons: (Gun, Sword),
    pub equipped: usize,
    pub kill_count: u128,
    pub damage: DamageSystem
}

impl Player {
    pub fn new() -> Self {
        Self {
            fields: BallEnt {
                position: Vector2::new(WIDTH/2.0, HEIGHT/2.0),
                direction: Vector2::new(0.0, 0.0),
                speed: 200.0,
                radius: 24.0,
                color: Color::BLACK
            },
            bullets: Vec::new(),
            weapons: (Gun::new(), Sword::new()),
            equipped: 0,
            kill_count: 0,
            damage: DamageSystem { 
                hitpoint: 5, 
                damage_cooldown: 30.0, 
                cooldown_value: 0.0 }
        }
    }
    
    pub fn supposed_to_be_dead(&self) -> bool {
        self.damage.hitpoint == 0
    }
}

pub struct Enemy { 
    pub fields: BallEnt, 
    pub damage: DamageSystem
}

impl Enemy {
    pub fn new() -> Self {
        let mut rng: ThreadRng = rng();
        let mut enemy = Self {
            fields: BallEnt {
                position: Vector2::new(
                    round_to_nearest(rng.random::<f32>()*(WIDTH+400.0)-200.0, 0.0, WIDTH), 
                    round_to_nearest(rng.random::<f32>()*(HEIGHT-400.0)+200.0, HEIGHT, 0.0)
                ),
                direction: Vector2::new(0.0, 0.0),
                speed: 125.0,
                radius: rng.random_range(18.0..26.0),
                color: Color::RED
            },
            damage: DamageSystem { 
                hitpoint: 0, 
                damage_cooldown: 2.0, 
                cooldown_value: 0.0 }
        };
        enemy.damage.hitpoint = if enemy.fields.radius - 18.0 < 4.0 { 1 } else { 2 };
        enemy
    }
}

pub fn enemy_handler(rl: &RaylibHandle, enemies: &mut Vec<Enemy>, spawn_cooldown: &mut f32, player: &mut Player) {
    if *spawn_cooldown <= 0.0 {
        let enemy = Enemy::new();
        enemies.push(enemy);
        if player.supposed_to_be_dead() { *spawn_cooldown = 4. } else { *spawn_cooldown = 10. }
        
    }
    *spawn_cooldown = (*spawn_cooldown - 10.0 * rl.get_frame_time()).max(0.0);
    enemies.retain_mut(|enemy| {
        let sword = &player.weapons.1;
        enemy.damage.cooldown_value = (enemy.damage.cooldown_value - 10.0 * rl.get_frame_time()).max(0.0);
        player.damage.cooldown_value = (player.damage.cooldown_value - 10.0 * rl.get_frame_time()).max(0.0);
        if player.damage.cooldown_value <= 0.0 && check_collision_circles(player.fields.position, player.fields.radius, enemy.fields.position, enemy.fields.radius) {
            if player.supposed_to_be_dead() {
                unsafe {
                    ffi::CloseWindow();
                }
            }
            player.damage.hitpoint -= 1;
            player.damage.cooldown_value = player.damage.damage_cooldown;
        }
        if enemy.damage.cooldown_value <= 0.0 
            && (player.bullets
                    .iter_mut()
                    .any(|bullet| { 
                        if check_collision_circles(enemy.fields.position, enemy.fields.radius, bullet.fields.position, bullet.fields.radius) {
                            bullet.hit_enemy = true;
                            true
                        } else { false }
                    })
                || (sword.fields.check_collision_circle_recpro(enemy.fields.position, enemy.fields.radius) && player.equipped == 1 && sword.is_swinging)) {
                enemy.damage.hitpoint -= 1;
                enemy.damage.cooldown_value = enemy.damage.damage_cooldown;
            }
        if enemy.damage.hitpoint > 0 { true } else { 
            player.kill_count += 1;
            false 
        }});
    for enemy in enemies {
        let angle_dir = enemy.fields.position.angle_to(player.fields.position);
        enemy.fields.direction = Vector2 { x: (angle_dir.cos()), y: (angle_dir.sin()) };
        let velocity = Vector2::scale_by(&enemy.fields.direction.normalized(), enemy.fields.speed * rl.get_frame_time());
        enemy.fields.position = Vector2::add(enemy.fields.position, velocity);
    }
}