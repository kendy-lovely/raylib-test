use crate::basic::{BallEnt};
use crate::weapons::{Gun, Sword};
use crate::utils::{Cooldown, DamageSystem, Prompt, round_to_nearest};
use rand::rng;
use raylib::{color::Color, prelude::*};
use rand::{Rng, rngs::ThreadRng};
use std::{ops::Add};
use crate::{WIDTH, HEIGHT};

pub struct Level {
    pub enemies: Vec<Enemy>,
    pub enemy_cooldown: Cooldown,
    pub prompt: Prompt
}

pub struct Player { 
    pub fields: BallEnt,
    pub level: u128,
    pub weapons: (Gun, Sword),
    pub equipped: usize,
    pub kill_count: u128,
    pub damage: DamageSystem,
    pub hit_by: Enemy
}

impl Player {
    pub fn new() -> Self {
        Self {
            fields: BallEnt {
                position: Vector2::new(WIDTH/2.0, HEIGHT/2.0),
                direction: Vector2::new(0.0, 0.0),
                speed: 200.0,
                radius: 24.0,
                color: Color::WHITE
            },
            level: 0,
            weapons: (Gun::new(), Sword::new()),
            equipped: 0,
            kill_count: 0,
            damage: DamageSystem { 
                hitpoint: 5,
                damage_cooldown: Cooldown {
                    cooldown: 30.0,
                    cooldown_value: 0.0
                } },
            hit_by: Enemy::new(&0)
        }
    }
    
    pub fn supposed_to_be_dead(&self) -> bool {
        self.damage.hitpoint == 0
    }
}

#[derive(Clone)]
pub struct Enemy { 
    pub fields: BallEnt, 
    pub damage: DamageSystem
}

impl Enemy {
    pub fn new(enemies_killed: &u128) -> Self {
        const INTERVAL: u128 = 4;
        const MIN_SIZE: f32 = 18.0;
        const ENEMIES_KILLED_PER_INCREASE: f32 = 20.0;
        let intervals_to_max: u128 = 1 + (*enemies_killed as f32 / ENEMIES_KILLED_PER_INCREASE) as u128;
        let mut rng: ThreadRng = rng();
        let mut enemy = Self {
            fields: BallEnt {
                position: Vector2::new(
                    round_to_nearest(rng.random::<f32>()*(WIDTH+400.0)-200.0, 0.0, WIDTH), 
                    round_to_nearest(rng.random::<f32>()*(HEIGHT-400.0)+200.0, HEIGHT, 0.0)
                ),
                direction: Vector2::new(0.0, 0.0),
                speed: 125.0,
                radius: rng.random_range(MIN_SIZE..=MIN_SIZE + (INTERVAL * intervals_to_max) as f32),
                color: Color::RED
            },
            damage: DamageSystem { 
                hitpoint: 0, 
                damage_cooldown: Cooldown { 
                    cooldown: 2.0, 
                    cooldown_value: 0.0 
                }
            }
        };
        enemy.damage.hitpoint = ((enemy.fields.radius - MIN_SIZE) / INTERVAL as f32 + 0.001).ceil() as u8;
        enemy
    }
}

pub fn player_handler(rl: &RaylibHandle, player: &mut Player, input: &Vector2, level: &mut Level) -> i32 {
    let mut shake: i32 = 0;
    let mut velocity: Vector2 = input.normalized().scale_by(player.fields.speed * rl.get_frame_time()).scale_by((player.damage.damage_cooldown.cooldown_value / 5.0).max(1.0));

    if player.kill_count as f32 % 20.0 == 0.0 && player.kill_count != 0 && !level.prompt.has_chosen { level.prompt.prompt(&player) }

    for enemy in &level.enemies {
        
        player.damage.damage_cooldown.cooldown_value = (player.damage.damage_cooldown.cooldown_value - 10.0 * rl.get_frame_time()).max(0.0);
        if player.damage.damage_cooldown.cooldown_value <= 0.0 && check_collision_circles(player.fields.position, player.fields.radius, enemy.fields.position, enemy.fields.radius) {
            if player.supposed_to_be_dead() { unsafe { ffi::CloseWindow() } }
            player.damage.hitpoint = player.damage.hitpoint.saturating_sub(1);
            player.damage.damage_cooldown.cooldown_value = player.damage.damage_cooldown.cooldown;
            player.hit_by = enemy.clone();
        }
    }
    if player.damage.damage_cooldown.cooldown_value > 20.0 {
        let angle_to_enemy = player.fields.position.angle_to(player.hit_by.fields.position);
        let mut rng = rand::rng();
        let shake_value = ((player.damage.damage_cooldown.cooldown_value - 20.0) * 20.0) as i32;
        shake = rng.random_range(-shake_value..=shake_value);
        velocity = velocity - Vector2 {x: angle_to_enemy.cos(), y: angle_to_enemy.sin()}.scale_by(player.damage.damage_cooldown.cooldown_value / 5.0);
    }
    player.fields.position += velocity;
    shake
}

pub fn enemy_handler(rl: &RaylibHandle, level: &mut Level, player: &mut Player) {
    if level.enemy_cooldown.cooldown_value <= 0.0 {
        let enemy = Enemy::new(&player.kill_count);
        level.enemies.push(enemy);
        if player.supposed_to_be_dead() { level.enemy_cooldown.cooldown_value = level.enemy_cooldown.cooldown / 2.0 } else { level.enemy_cooldown.cooldown_value = level.enemy_cooldown.cooldown }
    }
    level.enemy_cooldown.cooldown_value = (level.enemy_cooldown.cooldown_value - 10.0 * rl.get_frame_time()).max(0.0);

    level.enemies.retain_mut(|enemy| {
        let (gun, sword) = &mut player.weapons;

        enemy.damage.damage_cooldown.cooldown_value = (enemy.damage.damage_cooldown.cooldown_value - 10.0 * rl.get_frame_time()).max(0.0);
        if enemy.damage.damage_cooldown.cooldown_value <= 0.0
            && gun.bullets
                    .iter_mut()
                    .any(|bullet| { 
                        if check_collision_circles(enemy.fields.position, enemy.fields.radius, bullet.fields.position, bullet.fields.radius) {
                            bullet.hit_enemy = true;
                            true
                        } else { false }
                    }) {
                enemy.damage.hitpoint -= 1;
                enemy.damage.damage_cooldown.cooldown_value = enemy.damage.damage_cooldown.cooldown;
                enemy.fields.radius -= 4.0;
        }

        if enemy.damage.damage_cooldown.cooldown_value <= 0.0
            && (sword.fields.check_collision_circle_recpro(enemy.fields.position, enemy.fields.radius) && player.equipped == 1 && sword.is_swinging) {
                enemy.damage.hitpoint = enemy.damage.hitpoint.saturating_sub(sword.damage.ceil() as u8);
                enemy.damage.damage_cooldown.cooldown_value = enemy.damage.damage_cooldown.cooldown;
                enemy.fields.radius -= 4.0 * sword.damage;
        }
        
        let angle_dir = enemy.fields.position.angle_to(player.fields.position);
        let velocity = Vector2::scale_by(&enemy.fields.direction.normalized(), enemy.fields.speed * rl.get_frame_time());
        enemy.fields.direction = Vector2 { x: (angle_dir.cos()), y: (angle_dir.sin()) };
        enemy.fields.position = Vector2::add(enemy.fields.position, velocity);

        if enemy.damage.hitpoint > 0 { true } else { 
            player.kill_count += 1;
            level.prompt.has_chosen = false;
            false 
        }});
}