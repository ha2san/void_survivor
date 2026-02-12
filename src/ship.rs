// =============================================================================
// VAISSEAU DU JOUEUR
// =============================================================================
//
// Ce fichier gère :
// - Les mouvements et rotations
// - Le bouclier énergétique
// - Le slow-motion
// - L'invincibilité après dégât
// - La traînée visuelle
//
// =============================================================================

use macroquad::prelude::*;
use std::collections::VecDeque;
use crate::consts::*;
use crate::game::Cooldown;

// -----------------------------------------------------------------------------
// STRUCTURE DU VAISSEAU
// -----------------------------------------------------------------------------
pub struct Ship {
    // Position et mouvement
    pub pos: Vec2,
    pub vel: Vec2,
    pub rot: f32,
    pub dir: Vec2,
    
    // État du joueur
    pub lives: i32,
    pub invincible_timer: Cooldown,
    
    // Bouclier
    pub shield_energy: Cooldown,
    pub shield_active: bool,
    
    // Slow-motion
    pub slowmo_energy: Cooldown,
    pub slowmo_active: bool,
    pub is_slowmo_available: bool,
    
    // Armes
    pub rapid_fire_timer: Cooldown,
    pub reverse_boost_timer: Cooldown,
    
    // Effets visuels
    pub trail: VecDeque<TrailPoint>,
}

// -----------------------------------------------------------------------------
// POINT DE TRAÎNÉE (EFFET VISUEL)
// -----------------------------------------------------------------------------
pub struct TrailPoint {
    pub pos: Vec2,
    pub life: f32,
}

// -----------------------------------------------------------------------------
// IMPLÉMENTATION DU VAISSEAU
// -----------------------------------------------------------------------------
impl Ship {
    /// Crée un nouveau vaisseau au centre de l'écran
    pub fn new() -> Self {
        Self {
            pos: vec2(screen_width() / 2., screen_height() / 2.),
            vel: Vec2::ZERO,
            rot: 0.0,
            dir: vec2(1.0, 0.0),
            lives: PLAYER_LIVES,
            invincible_timer: Cooldown::new(PLAYER_INVINCIBLE_TIME),
            shield_energy: Cooldown::full(MAX_SHIELD_TIME),
            shield_active: false,
            slowmo_energy: Cooldown::full(MAX_SLOWMO_TIME),
            slowmo_active: false,
            is_slowmo_available: true,
            rapid_fire_timer: Cooldown::new(7.0),
            reverse_boost_timer: Cooldown::new(0.5),
            trail: VecDeque::new(),
        }
    }

    /// Met à jour le vaisseau pour une frame
    /// Retourne le vecteur directionnel (pour les armes)
    pub fn update(&mut self, dt: f32) -> Vec2 {
        // Mise à jour des timers
        self.invincible_timer.tick(dt);
        self.rapid_fire_timer.tick(dt);
        self.reverse_boost_timer.tick(dt);

        // -----------------------------------------------------------------
        // GESTION DU BOUCLIER
        // -----------------------------------------------------------------
        self.shield_active = is_key_down(KEY_SHIELD) && self.shield_energy.time > 0.1;
        if self.shield_active {
            // Consommation d'énergie
            self.shield_energy.tick(dt * 1.5);
            //self.shield_energy.time = (self.shield_energy.time - dt * 1.5).max(0.0);
        } else {
            self.shield_energy.increase(dt * 0.6);      // Recharge lente du bouclier
        }

        // -----------------------------------------------------------------
        // GESTION DU SLOW-MOTION
        // -----------------------------------------------------------------
        self.slowmo_active = is_key_down(KEY_SLOWMO) && 
                             self.slowmo_energy.time > 0.1 && 
                             self.is_slowmo_available;
                             
        if self.slowmo_active {
            // Consommation d'énergie
            self.slowmo_energy.tick(dt * 2.5);
            if self.slowmo_energy.time <= 0.0 {
                self.is_slowmo_available = false;
            }
        } else if self.slowmo_energy.time < self.slowmo_energy.max {
            // Recharge quand inactif
            self.slowmo_energy.increase(dt*0.4);
            if self.slowmo_energy.time >= self.slowmo_energy.max * 0.9 {
                self.is_slowmo_available = true;
            }
        }

        // -----------------------------------------------------------------
        // GESTION DES MOUVEMENTS
        // -----------------------------------------------------------------
        self.handle_input(dt);
        
        // -----------------------------------------------------------------
        // GESTION DE LA TRAÎNÉE
        // -----------------------------------------------------------------
        self.update_trail(dt);
        
        self.dir
    }

    /// Traite les entrées clavier pour le mouvement
    fn handle_input(&mut self, dt: f32) {
        // Rotation
        if is_key_down(KEY_LEFT) {
            self.rot -= 2.0 * dt;
        }
        if is_key_down(KEY_RIGHT) {
            self.rot += 2.0 * dt;
        }

        // Mise à jour du vecteur directionnel
        self.dir = vec2(self.rot.cos(), self.rot.sin());

        // Accélération
        let mut move_speed = 0.0;
        if is_key_down(KEY_UP) {
            move_speed = 500.0;
        }
        if is_key_down(KEY_DOWN) {
            move_speed = -300.0;
        }

        // Boost de recul (R)
        let reverse_boost = if is_key_down(KEY_REVERSE_BOOST) && !self.shield_active {
            self.reverse_boost_timer.reset();
            2.5
        } else {
            1.0
        };

        if move_speed != 0.0 {
            let effective_speed = if move_speed < 0.0 {
                move_speed * reverse_boost
            } else {
                move_speed
            };

            self.vel += self.dir * effective_speed * dt;
        }

        // Frottement
        self.vel *= 0.97;
        
        // Mise à jour de la position
        self.pos += self.vel * dt;
        
        // Téléportation aux bords de l'écran (wrap-around)
        self.pos.x = self.pos.x.rem_euclid(screen_width());
        self.pos.y = self.pos.y.rem_euclid(screen_height());
    }

    /// Met à jour la traînée visuelle du vaisseau
    fn update_trail(&mut self, dt: f32) {
        // Ajout d'un nouveau point
        self.trail.push_back(TrailPoint {
            pos: self.pos,
            life: 0.5,
        });

        // Vieillissement des points
        for tp in self.trail.iter_mut() {
            tp.life -= dt;
        }

        // Suppression des points trop vieux
        self.trail.retain(|tp| tp.life > 0.0);
    }

    /// Inflige des dégâts au vaisseau (retourne true si dégât effectif)
    pub fn take_damage(&mut self) -> bool {
        if self.invincible_timer.ready() && !self.shield_active {
            self.lives -= 1;
            self.invincible_timer.reset();
            true
        } else {
            false
        }
    }

    /// Vérifie si le vaisseau est invincible
    pub fn is_invincible(&self) -> bool {
        !self.invincible_timer.ready()
    }

    /// Retourne le facteur de temps (1.0 = normal, 0.3 = slow-mo)
    pub fn get_slow_time_factor(&self) -> f32 {
        if self.slowmo_active {
            0.3
        } else {
            1.0
        }
    }
}
