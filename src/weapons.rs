// =============================================================================
// SYSTÈME D'ARMES
// =============================================================================
//
// Ce fichier gère toutes les armes du joueur :
// - Tir normal (mitrailleuse)
// - Missiles à tête chercheuse
// - Laser perçant
//
// =============================================================================

use macroquad::prelude::*;
use crate::consts::*;
use crate::ship::Ship;
use crate::game::{World, Cooldown};
use crate::enemies::*;
use crate::effects::EffectSystem;
use crate::events::EventQueue;

// -----------------------------------------------------------------------------
// SYSTÈME PRINCIPAL D'ARMES
// -----------------------------------------------------------------------------
pub struct WeaponSystem {
    // Tir normal
    pub fire_cooldown: Cooldown,
    
    // Missiles
    pub missile_ammo: i32,
    pub missile_cooldown: Cooldown,
    pub missiles: Vec<HomingMissile>,
    pub ammo_regen_timer: f32,
    
    // Laser
    pub laser: Laser,
}

// -----------------------------------------------------------------------------
// MISSILE À TÊTE CHERCHEUSE
// -----------------------------------------------------------------------------
pub struct HomingMissile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
    pub speed: f32,
    pub hit: bool,  // Évite les collisions multiples
}

impl HomingMissile {
    /// Crée un nouveau missile
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Self {
            pos,
            vel: dir * MISSILE_SPEED,
            life: MISSILE_LIFE,
            speed: MISSILE_SPEED,
            hit: false,
        }
    }

    /// Met à jour la position du missile
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.life -= dt;
    }

    /// Vérifie si le missile est toujours actif
    pub fn is_alive(&self) -> bool {
        self.life > 0.0 
            && !self.hit
            && self.pos.x > -100.0 
            && self.pos.x < screen_width() + 100.0
            && self.pos.y > -100.0 
            && self.pos.y < screen_height() + 100.0
    }

    /// Poursuite de la cible (homing)
    pub fn homing(&mut self, target: Vec2, dt: f32) {
        let desired_dir = (target - self.pos).normalize_or_zero();
        if desired_dir != Vec2::ZERO {
            let current_dir = self.vel.normalize_or_zero();
            let new_dir = current_dir
                .lerp(desired_dir, MISSILE_TURN_SPEED * dt)
                .normalize_or_zero();
            self.vel = new_dir * self.speed;
        }
    }
}

// -----------------------------------------------------------------------------
// LASER PERÇANT
// -----------------------------------------------------------------------------
pub struct Laser {
    pub active: bool,
    pub start_pos: Vec2,
    pub direction: Vec2,
    pub timer: Cooldown,
    pub cooldown: Cooldown,
}

impl Laser {
    /// Crée un nouveau laser (désactivé par défaut)
    pub fn new() -> Self {
        Self {
            active: false,
            start_pos: Vec2::ZERO,
            direction: Vec2::ZERO,
            timer: Cooldown::new(LASER_DURATION),
            cooldown: Cooldown::new(LASER_COOLDOWN),
        }
    }

    /// Déclenche un tir laser
    pub fn fire(&mut self, ship_pos: Vec2, ship_dir: Vec2) {
        self.active = true;
        self.timer.reset();
        self.cooldown.reset();
        self.start_pos = ship_pos;
        self.direction = ship_dir;
    }

    /// Met à jour l'état du laser
    pub fn update(&mut self, dt: f32, ship_pos: Vec2, ship_dir: Vec2) {
        if self.active {
            self.timer.tick(dt);
            // Mise à jour en temps réel : le laser suit le vaisseau
            self.start_pos = ship_pos;
            self.direction = ship_dir;
            
            if self.timer.ready() {
                self.active = false;
            }
        } else {
            self.cooldown.tick(dt);
        }
    }
}

// -----------------------------------------------------------------------------
// IMPLÉMENTATION DU SYSTÈME D'ARMES
// -----------------------------------------------------------------------------
impl WeaponSystem {
    /// Crée un nouveau système d'armes
    pub fn new() -> Self {
        Self {
            fire_cooldown: Cooldown::new(0.2),
            missile_ammo: MISSILE_AMMO_MAX,
            missile_cooldown: Cooldown::new(MISSILE_COOLDOWN_MAX),
            missiles: Vec::new(),
            laser: Laser::new(),
            ammo_regen_timer: 0.0,
        }
    }

    /// Met à jour toutes les armes
    pub fn update(
        &mut self, 
        dt: f32, 
        ship: &mut Ship, 
        world: &mut World, 
        effects: &mut EffectSystem,
        event_queue: &mut EventQueue,
    ) {
        self.fire_cooldown.tick(dt);
        self.missile_cooldown.tick(dt);
        
        let ship_dir = vec2(ship.rot.cos(), ship.rot.sin());
        
        // Tir normal
        self.handle_shooting(dt, ship, ship_dir, world, effects, event_queue);
        
        // Tir missile
        self.handle_missile_launch(dt, ship, ship_dir, world, effects, event_queue);
        
        // Recharge automatique des missiles
        self.regen_missile_ammo(dt);
        
        // Tir laser
        self.handle_laser(dt, ship, ship_dir, event_queue);
        
        // Mise à jour du laser
        self.laser.update(dt, ship.pos, ship_dir);
    }

    /// Gère le tir normal (ESPACE)
    fn handle_shooting(
        &mut self, 
        _dt: f32,                    // Non utilisé (conservé pour signature)
        ship: &mut Ship, 
        ship_dir: Vec2, 
        world: &mut World, 
        effects: &mut EffectSystem,
        _event_queue: &mut EventQueue, // Non utilisé (conservé pour extension)
    ) {
        let fire_rate = if ship.rapid_fire_timer.time > 0.0 { 0.1 } else { 0.2 };
        
        if (is_key_down(KEY_SHOOT) || is_key_pressed(KEY_SHOOT)) 
           && !ship.shield_active 
           && self.fire_cooldown.ready() {
            
            // Taille de balle différente selon le power-up
            let bullet_size = if ship.rapid_fire_timer.time > 0.0 { 2.0 } else { 3.0 };
            
            // Création de la balle
            world.bullets.push(Bullet {
                pos: ship.pos + ship_dir * 15.0,
                vel: ship_dir * BULLET_SPEED,
                enemy: false,
                size: bullet_size,
                life: 2.0,
            });

            // Recul
            ship.vel += -ship_dir * 30.0;
            self.fire_cooldown.time = fire_rate;

            // Effet visuel
            effects.create_muzzle_flash(ship.pos + ship_dir * 10.0, ship_dir);
        }
    }

    /// Gère le tir de missile à tête chercheuse (F)
    fn handle_missile_launch(
        &mut self, 
        _dt: f32,                    // Non utilisé
        ship: &mut Ship, 
        ship_dir: Vec2, 
        _world: &mut World,          // Non utilisé
        effects: &mut EffectSystem,
        _event_queue: &mut EventQueue, // Non utilisé
    ) {
        if is_key_pressed(KEY_MISSILE) 
           && !ship.shield_active 
           && self.missile_ammo > 0 
           && self.missile_cooldown.ready() {
            
            // Création du missile
            self.missiles.push(HomingMissile::new(
                ship.pos + ship_dir * 20.0,
                ship_dir,
            ));
            
            // Consommation munition
            self.missile_ammo -= 1;
            self.missile_cooldown.reset();
            
            // Recul
            ship.vel += -ship_dir * 50.0;
            
            // Effet visuel
            effects.create_missile_launch(ship.pos + ship_dir * 20.0, ship_dir);
        }
    }

    /// Recharge automatique des missiles
    fn regen_missile_ammo(&mut self, dt: f32) {
        if self.missile_ammo < MISSILE_AMMO_MAX {
            self.ammo_regen_timer += dt;
            if self.ammo_regen_timer >= 1.0 {
                self.missile_ammo = (self.missile_ammo + 1).min(MISSILE_AMMO_MAX);
                self.ammo_regen_timer = 0.0;
            }
        }
    }

    /// Gère le tir laser (L)
    fn handle_laser(
        &mut self, 
        _dt: f32,                    // Non utilisé
        ship: &mut Ship, 
        ship_dir: Vec2, 
        event_queue: &mut EventQueue,
    ) {
        if is_key_pressed(KEY_LASER) 
           && !ship.shield_active 
           && self.laser.cooldown.ready() 
           && !self.laser.active {
            
            // Activation du laser
            self.laser.fire(ship.pos, ship_dir);
            
            // Recul
            ship.vel += -ship_dir * 80.0;
            
            // Événement pour les effets visuels
            event_queue.push(crate::events::Event::LaserFired {
                pos: ship.pos,
                dir: ship_dir,
            });
        }
    }

    /// Met à jour tous les missiles (mouvement + collisions)
    pub fn update_missiles(
        &mut self, 
        dt: f32, 
        world: &mut World, 
        effects: &mut EffectSystem, 
        score: &mut i32, 
        combo: &mut i32, 
        combo_timer: &mut Cooldown, 
        enemies_killed: &mut i32, 
        _event_queue: &mut EventQueue,
    ) {
        // Mise à jour du mouvement et homing
        for missile in self.missiles.iter_mut() {
            missile.update(dt);
            
            // Recherche de la cible la plus proche
            let mut target_pos = None;
            let mut min_dist = f32::INFINITY;
            
            // Priorité aux drones
            for drone in world.drones.iter() {
                if drone.hp > 0 {
                    let dist = missile.pos.distance_squared(drone.pos);
                    if dist < min_dist {
                        min_dist = dist;
                        target_pos = Some(drone.pos);
                    }
                }
            }


            if target_pos == None {
                // Sinon, astéroïdes
                for asteroid in world.asteroids.iter() {
                    if asteroid.radius > 10.0 {
                        let dist = missile.pos.distance_squared(asteroid.pos);
                        if dist < min_dist {
                            min_dist = dist;
                            target_pos = Some(asteroid.pos);
                        }
                    }
                }
            }

            // Poursuite de la cible
            if let Some(target) = target_pos {
                missile.homing(target, dt);
            }
        }

        // Gestion des collisions (avec suppression optimisée)
        let mut i = 0;
        while i < self.missiles.len() {
            let missile = &self.missiles[i];
            let mut hit = false;

            // Collision avec les drones
            for drone in world.drones.iter_mut() {
                if drone.hp > 0 && missile.pos.distance(drone.pos) < 15.0 {
                    drone.hp -= 2;  // Missile = dégâts importants
                    *score += 20;
                    *combo += 1;
                    combo_timer.reset();

                    effects.create_explosion(missile.pos, ORANGE, 12);

                    if drone.hp <= 0 {
                        *enemies_killed += 1;
                        *score += 50;
                        if rand::gen_range(0.0, 1.0) < POWERUP_SPAWN_CHANCE {
                            world.powerups.push(PowerUp::new(drone.pos));
                        }
                    }

                    hit = true;
                    break;
                }
            }

            // Collision avec les astéroïdes
            if !hit {
                for asteroid in world.asteroids.iter_mut() {
                    if asteroid.radius > 10.0 && missile.pos.distance(asteroid.pos) < asteroid.radius + 10.0 {
                        asteroid.radius -= 15.0;
                        *score += 15;
                        *combo += 1;
                        combo_timer.reset();

                        effects.create_explosion(missile.pos, ORANGE, 12);

                        if asteroid.radius <= 10.0 {
                            if rand::gen_range(0.0, 1.0) < POWERUP_SPAWN_CHANCE {
                                world.powerups.push(PowerUp::new(asteroid.pos));
                            }
                        }

                        hit = true;
                        break;
                    }
                }
            }

            // Suppression si touché (swap_remove = O(1))
            if hit {
                self.missiles.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Met à jour le laser (dégâts et collisions)
    pub fn update_laser(
        &mut self, 
        _dt: f32,                    // Non utilisé
        _ship: &Ship,               // Non utilisé
        world: &mut World, 
        effects: &mut EffectSystem, 
        score: &mut i32, 
        enemies_killed: &mut i32, 
        _event_queue: &mut EventQueue,
    ) {
        if !self.laser.active {
            return;
        }

        let beam_start = self.laser.start_pos;
        let beam_dir = self.laser.direction;
        let mut hit_count = 0;

        // Drones
        for drone in world.drones.iter_mut() {
            if hit_count >= LASER_MAX_PENETRATION { break; }
            if drone.hp <= 0 { continue; }

            let to_drone = drone.pos - beam_start;
            let proj = to_drone.dot(beam_dir);

            if proj > 0.0 && proj < LASER_RANGE {
                let perp = (beam_start + beam_dir * proj).distance(drone.pos);
                if perp < 15.0 {
                    drone.hp -= LASER_DAMAGE;
                    hit_count += 1;

                    effects.create_explosion(drone.pos, RED, 6);

                    if drone.hp <= 0 {
                        *score += 50;
                        *enemies_killed += 1;
                        if rand::gen_range(0.0, 1.0) < POWERUP_SPAWN_CHANCE {
                            world.powerups.push(PowerUp::new(drone.pos));
                        }
                    }
                }
            }
        }

        // Astéroïdes
        for asteroid in world.asteroids.iter_mut() {
            if hit_count >= LASER_MAX_PENETRATION { break; }

            let to_ast = asteroid.pos - beam_start;
            let proj = to_ast.dot(beam_dir);

            if proj > 0.0 && proj < LASER_RANGE {
                let perp = (beam_start + beam_dir * proj).distance(asteroid.pos);
                if perp < asteroid.radius + 10.0 {
                    asteroid.radius -= 10.0;
                    hit_count += 1;

                    effects.create_explosion(asteroid.pos, RED, 6);

                    if asteroid.radius <= 10.0 
                        && rand::gen_range(0.0, 1.0) < POWERUP_SPAWN_CHANCE {
                            world.powerups.push(PowerUp::new(asteroid.pos));
                    }
                }
            }
        }
    }
}
