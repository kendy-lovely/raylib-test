use raylib::{RaylibHandle, ffi::KeyboardKey};
use crate::entities::{Level, Player};

pub struct DamageSystem {
    pub hitpoint: u8,
    pub damage_cooldown: Cooldown
}

pub struct Cooldown {
    pub cooldown: f32,
    pub cooldown_value: f32
}

pub struct Prompt {
    pub appear: bool,
    pub text: (String, String),
    pub has_chosen: bool
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            appear: false,
            text: (String::new(), String::new()),
            has_chosen: false
        }
    }
    pub fn prompt(&mut self, player: &Player) {
        self.appear = true;
        let (gun, sword) = &player.weapons;
        match gun.level {
            0 => self.text.0 = format!("Get new weapon\nGun; Q"),
            _ => self.text.0 = format!("Upgrade Gun\nto level {}; Q", gun.level + 1)
        }
        match sword.level {
            0 => self.text.1 = format!("Get new weapon\nSword; E"),
            _ => self.text.1 = format!("Upgrade Sword\nto level {}; E", sword.level + 1)
        }
        
    }
    pub fn select(level: &mut Level, rl: &mut RaylibHandle, player: &mut Player) {
        let (gun, sword) = &mut player.weapons;
        let input = rl.get_key_pressed();
            match input {
                Some(i) => { match i {
                    KeyboardKey::KEY_Q => { 
                        gun.add_level(); 
                        level.prompt.has_chosen = true; 
                        level.prompt.appear = false; 
                        player.damage.hitpoint += 1;
                        player.level += 1;
                        level.enemy_cooldown.cooldown -= player.level as f32 / 3.5
                    }
                    KeyboardKey::KEY_E => { 
                        sword.add_level(); 
                        level.prompt.has_chosen = true; 
                        level.prompt.appear = false; 
                        player.damage.hitpoint += 1;
                        player.level += 1;
                        level.enemy_cooldown.cooldown -= player.level as f32 / 3.5
                    }
                    _ => {}
                }}
                None => {}
            }
    }
}

pub fn round_to_nearest(x: f32, a: f32, b: f32) -> f32 {
    let (min_val, max_val) = (a.min(b), a.max(b));
    let diff_min = (x - min_val).abs();
    let diff_max = (x - max_val).abs();
    if x > min_val && x < max_val {
        if diff_min > diff_max { b } else if diff_min < diff_max { a } else { a }
    } else { x }
}