// =============================================================================
// SYSTÈME DE COLLISIONS
// =============================================================================
//
// Ce fichier centralise TOUTES les règles de collisions :
// - Balles vs ennemis
// - Balles ennemies vs joueur
// - Vaisseau vs astéroïdes
// - Vaisseau vs drones
// - Power-ups vs joueur
//
// =============================================================================

use macroquad::prelude::*;
use crate::consts::*;
use crate::game::*;
use crate::enemies::*;
use crate::effects::EffectSystem;
use crate::events::EventQueue;

// -----------------------------------------------------------------------------
// COLLISIONS BALLES AMIES VS ENNEMIS
// -----------------------------------------------------------------------------
pub fn handle_friendly_bullet_collisions(
    bullets: &mut Vec<Bullet>,
    asteroids: &mut Vec<Asteroid>,
    drones: &mut Vec<Drone>,
    score: &mut i32,
    combo: &mut i32,
    combo_timer: &mut Cooldown,
    enemies_killed: &mut i32,
    effects: &mut EffectSystem,
    powerups: &mut Vec<PowerUp>,
    _event_queue: &mut EventQueue,  // Non utilisé actuellement
) {
    let mut i = 0;
    while i < bullets.len() {
        let bullet = &bullets[i];
        
        // Ignorer les balles ennemies
        if bullet.enemy {
            i += 1;
            continue;
        }

        let mut bullet_hit = false;

        // Collision avec les astéroïdes
        for asteroid in asteroids.iter_mut() {
            if bullet.pos.distance(asteroid.pos) < asteroid.radius {
                asteroid.radius -= 10.0;
                *score += 10;
                *combo += 1;
                combo_timer.reset();
                
                effects.create_explosion(asteroid.pos, WHITE, 10);
                
                // Chance de spawn power-up
                if asteroid.radius <= 10.0 
                   && rand::gen_range(0.0, 1.0) < POWERUP_SPAWN_CHANCE {
                    powerups.push(PowerUp::new(asteroid.pos));
                }
                
                bullet_hit = true;
                break;
            }
        }

        // Collision avec les drones (si pas déjà touché)
        if !bullet_hit {
            for drone in drones.iter_mut() {
                if bullet.pos.distance(drone.pos) < 15.0 {
                    drone.hp -= 1;
                    *score += 50;
                    *combo += 1;
                    combo_timer.reset();
                    
                    effects.create_explosion(drone.pos, RED, 8);
                    
                    if drone.hp <= 0 {
                        *enemies_killed += 1;
                        *score += 50;
                        
                        effects.create_explosion(drone.pos, ORANGE, 15);
                        
                        if rand::gen_range(0.0, 1.0) < POWERUP_SPAWN_CHANCE {
                            powerups.push(PowerUp::new(drone.pos));
                        }
                    }
                    
                    bullet_hit = true;
                    break;
                }
            }
        }

        // Suppression de la balle si elle a touché quelque chose
        if bullet_hit {
            bullets.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

// -----------------------------------------------------------------------------
// COLLISIONS BALLES ENNEMIES VS VAISSEAU
// -----------------------------------------------------------------------------
pub fn handle_enemy_bullet_collisions(
    bullets: &mut Vec<Bullet>,
    ship_pos: &Vec2,
    invincible_time: f32,
    shield_active: bool,
    lives: &mut i32,
    ship_invincible: &mut Cooldown,
    shake: &mut ScreenShake,
    effects: &mut EffectSystem,
    state: &mut GameState,
    event_queue: &mut EventQueue,
) {
    let mut i = 0;
    while i < bullets.len() {
        let bullet = &bullets[i];
        
        // Ignorer les balles alliées
        if !bullet.enemy {
            i += 1;
            continue;
        }

        // Collision avec le vaisseau
        if bullet.pos.distance(*ship_pos) < 12.0 && invincible_time <= 0.0 {
            if !shield_active {
                *lives -= 1;
                ship_invincible.reset();
                shake.trigger(0.3);
                
                effects.create_explosion(*ship_pos, RED, 10);
                
                if *lives <= 0 {
                    *state = GameState::GameOver;
                    event_queue.push(crate::events::Event::GameOver);
                }
            } else {
                // Bouclier : la balle est détruite sans dégât
                effects.create_explosion(bullet.pos, SKYBLUE, 6);
            }
            
            bullets.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

// -----------------------------------------------------------------------------
// COLLISIONS VAISSEAU VS ASTÉROÏDES
// -----------------------------------------------------------------------------
pub fn handle_ship_asteroid_collisions(
    asteroids: &mut Vec<Asteroid>,
    ship_pos: &Vec2,
    invincible_time: f32,
    shield_active: bool,
    lives: &mut i32,
    ship_invincible: &mut Cooldown,
    shake: &mut ScreenShake,
    score: &mut i32,
    effects: &mut EffectSystem,
    state: &mut GameState,
    event_queue: &mut EventQueue,
) {
    for asteroid in asteroids.iter_mut() {
        if ship_pos.distance(asteroid.pos) < asteroid.radius + 8.0 
           && invincible_time <= 0.0 {
            
            if shield_active {
                // Rebond sur le bouclier
                asteroid.vel = (asteroid.pos - *ship_pos).normalize() * 400.0;
                shake.trigger(0.3);
                *score += 5;
            } else {
                // Dégâts
                *lives -= 1;
                ship_invincible.reset();
                shake.trigger(0.5);
                
                effects.create_explosion(asteroid.pos, WHITE, 12);
                
                if *lives <= 0 {
                    *state = GameState::GameOver;
                    event_queue.push(crate::events::Event::GameOver);
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// COLLISIONS VAISSEAU VS DRONES
// -----------------------------------------------------------------------------
pub fn handle_ship_drone_collisions(
    drones: &mut Vec<Drone>,
    asteroids: &mut Vec<Asteroid>,
    ship_pos: &Vec2,
    invincible_time: f32,
    shield_active: bool,
    lives: &mut i32,
    ship_invincible: &mut Cooldown,
    shake: &mut ScreenShake,
    effects: &mut EffectSystem,
    state: &mut GameState,
    score: &mut i32,
    enemies_killed: &mut i32,
    event_queue: &mut EventQueue,
) {
    for drone in drones.iter_mut() {
        // Collision drone - astéroïde (dommages collatéraux)
        for asteroid in asteroids.iter_mut() {
            if drone.pos.distance(asteroid.pos) < 20.0 + asteroid.radius {
                drone.hp = 0;
                asteroid.radius -= 15.0;
                
                effects.create_explosion(drone.pos, ORANGE, 15);
                shake.trigger(0.2);
                
                if asteroid.radius <= 10.0 {
                    effects.create_explosion(asteroid.pos, WHITE, 12);
                }
                
                *enemies_killed += 1;
                *score += 50;
            }
        }

        // Collision drone - vaisseau
        if ship_pos.distance(drone.pos) < 20.0 && invincible_time <= 0.0 {
            if !shield_active {
                *lives -= 1;
                ship_invincible.reset();
                shake.trigger(0.5);
                drone.hp = 0;
                
                effects.create_explosion(drone.pos, RED, 15);
                
                if *lives <= 0 {
                    *state = GameState::GameOver;
                    event_queue.push(crate::events::Event::GameOver);
                }
            } else if shield_active {
                // Rebond sur le bouclier
                drone.vel = (drone.pos - *ship_pos).normalize() * 300.0;
                drone.hp -= 1;
                
                effects.create_explosion(drone.pos, SKYBLUE, 8);
            }
        }
    }
}

// -----------------------------------------------------------------------------
// COLLISIONS VAISSEAU VS POWER-UPS
// -----------------------------------------------------------------------------
pub fn handle_powerup_collisions(
    powerups: &mut Vec<PowerUp>,
    ship_pos: &Vec2,
    shield_energy: &mut Cooldown,
    lives: &mut i32,
    slowmo_energy: &mut Cooldown,
    rapid_fire_timer: &mut Cooldown,
    shake: &mut ScreenShake,
    effects: &mut EffectSystem,
    event_queue: &mut EventQueue,
) {
    let mut i = 0;
    while i < powerups.len() {
        if ship_pos.distance(powerups[i].pos) < 20.0 {
            let powerup = &powerups[i];
            
            // Application de l'effet selon le type
            match powerup.kind {
                PowerUpType::Shield => {
                    shield_energy.time = shield_energy.max;
                    effects.create_text_effect(powerup.pos, SKYBLUE);
                }
                PowerUpType::Life => {
                    *lives = (*lives + 1).min(5);
                    effects.create_text_effect(powerup.pos, GREEN);
                }
                PowerUpType::SlowTime => {
                    slowmo_energy.time = slowmo_energy.max;
                    effects.create_text_effect(powerup.pos, PURPLE);
                }
                PowerUpType::RapidFire => {
                    rapid_fire_timer.time = rapid_fire_timer.max;
                    effects.create_text_effect(powerup.pos, ORANGE);
                }
            }
            
            shake.trigger(0.2);
            event_queue.push(crate::events::Event::PowerUpSpawn { pos: powerup.pos });
            powerups.swap_remove(i);
        } else {
            i += 1;
        }
    }
}
