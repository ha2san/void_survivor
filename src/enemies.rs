// =============================================================================
// ENTITÉS ENNEMIES
// =============================================================================
//
// Ce fichier contient toutes les entités hostiles et items :
// - Astéroïdes
// - Drones (Sniper, Kamikaze, Bomber)
// - Balles ennemies
// - Power-ups (bonus)
//
// =============================================================================

use macroquad::prelude::*;
use crate::ship::Ship;
use crate::game::Cooldown;

// -----------------------------------------------------------------------------
// ASTÉROÏDE
// -----------------------------------------------------------------------------
pub struct Asteroid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
}

impl Asteroid {
    /// Crée un nouvel astéroïde
    pub fn new(pos: Vec2, difficulty: f32) -> Self {
        Self {
            pos,
            vel: vec2(
                rand::gen_range(-50., 50.),
                rand::gen_range(50., 150.) * difficulty.min(3.0),
            ),
            radius: rand::gen_range(15., 35.),
            rotation: 0.0,
            rotation_speed: rand::gen_range(-2.0, 2.0),
        }
    }

    /// Met à jour la position et la rotation
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.rotation += self.rotation_speed * dt;
    }

    /// Vérifie si l'astéroïde est toujours actif
    pub fn is_alive(&self) -> bool {
        self.radius > 10.0 
            && self.pos.y > -100.0 
            && self.pos.y < screen_height() + 100.0
            && self.pos.x > -100.0 
            && self.pos.x < screen_width() + 100.0
    }

    /// Inflige des dégâts et retourne true si détruit
    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.radius -= damage;
        self.radius <= 10.0
    }
}

// -----------------------------------------------------------------------------
// DRONE ENNEMI
// -----------------------------------------------------------------------------
#[derive(PartialEq, Clone)]
pub enum DroneType {
    Sniper,     // Tireur à distance
    Kamikaze,   // Se précipite sur le joueur
    Bomber,     // Lâche des bombes
}

pub struct Drone {
    pub pos: Vec2,
    pub vel: Vec2,
    pub cooldown: Cooldown,
    pub kind: DroneType,
    pub hp: i32,
    pub max_hp: i32,
}

impl Drone {
    /// Crée un nouveau drone avec un type aléatoire
    pub fn new(pos: Vec2, difficulty: f32, wave: i32) -> Self {
        let kind = match rand::gen_range(0, 3) {
            0 => DroneType::Sniper,
            1 => DroneType::Kamikaze,
            _ => DroneType::Bomber,
        };

        let cooldown = match kind {
            DroneType::Sniper => 2.0 / difficulty.min(3.0),
            DroneType::Bomber => 3.0,
            _ => 0.0,
        };

        let max_hp = 2 + (wave / 3);

        Self {
            pos,
            vel: Vec2::ZERO,
            cooldown: Cooldown::new(cooldown),
            kind,
            hp: max_hp,
            max_hp,
        }
    }

    /// Met à jour le comportement selon le type
    pub fn update(&mut self, dt: f32, ship_pos: &Vec2) {
        self.cooldown.tick(dt);
        
        match self.kind {
            DroneType::Kamikaze => {
                // Poursuite agressive
                let to_player = (*ship_pos - self.pos).normalize();
                self.vel = self.vel.lerp(to_player * 40.0, 0.1);
                self.pos += self.vel * dt;
            }
            DroneType::Sniper => {
                // Descente lente
                self.pos.y += 20.0 * dt;
            }
            DroneType::Bomber => {
                // Déplacement horizontal vers le joueur
                self.pos.x += (ship_pos.x - self.pos.x).signum() * 80.0 * dt;
                self.pos.y += 30.0 * dt;
            }
        }
    }

    /// Tire sur le joueur (retourne Option<Bullet>)
    pub fn shoot(&mut self, _dt: f32, ship: &Ship) -> Option<Bullet> {
        if self.cooldown.ready() {
            match self.kind {
                DroneType::Sniper => {
                    // Tir avec avance (lead)
                    let lead_dir = (ship.pos + ship.vel * 0.3 - self.pos).normalize();
                    self.cooldown.time = 2.5;
                    Some(Bullet {
                        pos: self.pos,
                        vel: lead_dir * 400.0,
                        enemy: true,
                        size: 4.0,
                        life: 3.0,
                    })
                }
                DroneType::Bomber => {
                    // Tir vertical
                    self.cooldown.time = 3.0;
                    Some(Bullet {
                        pos: self.pos,
                        vel: vec2(0.0, 150.0),
                        enemy: true,
                        size: 6.0,
                        life: 3.0,
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Vérifie si le drone est toujours actif
    pub fn is_alive(&self) -> bool {
        self.hp > 0 
            && self.pos.y > -50.0 
            && self.pos.y < screen_height() + 50.0
            && self.pos.x > -50.0 
            && self.pos.x < screen_width() + 50.0
    }

    /// Pourcentage de vie restant (pour barre de vie)
    pub fn health_percent(&self) -> f32 {
        self.hp as f32 / self.max_hp as f32
    }
}

// -----------------------------------------------------------------------------
// BALLE (ALLIÉE OU ENNEMIE)
// -----------------------------------------------------------------------------
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub enemy: bool,    // true = ennemi, false = allié
    pub size: f32,
    pub life: f32,      // Durée de vie avant disparition
}

impl Bullet {
    /// Met à jour la position
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.life -= dt;
    }

    /// Vérifie si la balle est toujours active
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
            && self.pos.y > -50.0 
            && self.pos.y < screen_height() + 50.0
            && self.pos.x > -50.0 
            && self.pos.x < screen_width() + 50.0
    }
}

// -----------------------------------------------------------------------------
// POWER-UP (BONUS)
// -----------------------------------------------------------------------------
#[derive(Clone)]
pub enum PowerUpType {
    Shield,     // Bouclier plein
    Life,       // Vie supplémentaire
    SlowTime,   // Slow-mo plein
    RapidFire,  // Tir rapide temporaire
}

pub struct PowerUp {
    pub pos: Vec2,
    pub vel: Vec2,
    pub kind: PowerUpType,
    pub life: f32,
}

impl PowerUp {
    /// Crée un nouveau power-up de type aléatoire
    pub fn new(pos: Vec2) -> Self {
        let kind = match rand::gen_range(0, 4) {
            0 => PowerUpType::Shield,
            1 => PowerUpType::Life,
            2 => PowerUpType::SlowTime,
            _ => PowerUpType::RapidFire,
        };

        Self {
            pos,
            vel: vec2(rand::gen_range(-30., 30.), rand::gen_range(-30., 30.)),
            kind,
            life: 10.0,
        }
    }

    /// Met à jour la position
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.life -= dt;
        self.vel.y += 50.0 * dt;  // Gravité
    }

    /// Vérifie si le power-up est toujours actif
    pub fn is_alive(&self) -> bool {
        self.life > 0.0 && self.pos.y < screen_height() + 50.0
    }

    /// Applique l'effet du power-up
    pub fn apply(&self, ship: &mut crate::ship::Ship, _weapons: &mut crate::weapons::WeaponSystem) {
        match self.kind {
            PowerUpType::Shield => {
                ship.shield_energy.time = ship.shield_energy.max;
            }
            PowerUpType::Life => {
                ship.lives = (ship.lives + 1).min(5);
            }
            PowerUpType::SlowTime => {
                ship.slowmo_energy.time = ship.slowmo_energy.max;
            }
            PowerUpType::RapidFire => {
                ship.rapid_fire_timer.time = ship.rapid_fire_timer.max;
            }
        }
    }

    /// Couleur associée au type (pour le rendu)
    pub fn color(&self) -> Color {
        match self.kind {
            PowerUpType::Shield => SKYBLUE,
            PowerUpType::Life => GREEN,
            PowerUpType::SlowTime => PURPLE,
            PowerUpType::RapidFire => ORANGE,
        }
    }
}
