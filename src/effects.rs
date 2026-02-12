// =============================================================================
// SYSTÈME D'EFFETS VISUELS
// =============================================================================
//
// Ce fichier gère tous les effets visuels :
// - Particules d'explosion
// - Traînée du vaisseau
// - Flashs de tir
// - Effets de texte (power-ups)
//
// =============================================================================

use macroquad::prelude::*;
use std::collections::VecDeque;
use crate::events::EventQueue;
use crate::events::Event;

// -----------------------------------------------------------------------------
// SYSTÈME D'EFFETS
// -----------------------------------------------------------------------------
pub struct EffectSystem {
    pub particles: Vec<Particle>,
    pub trail: VecDeque<TrailPoint>,
}

impl EffectSystem {
    /// Crée un nouveau système d'effets
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            trail: VecDeque::new(),
        }
    }

    /// Vide tous les effets
    pub fn clear(&mut self) {
        self.particles.clear();
        self.trail.clear();
    }

    /// Met à jour toutes les particules et la traînée
    pub fn update(&mut self, dt: f32) {
        // Particules
        for p in self.particles.iter_mut() {
            p.pos += p.vel * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        // Traînée
        for tp in self.trail.iter_mut() {
            tp.life -= dt;
        }
        self.trail.retain(|tp| tp.life > 0.0);
    }

    /// Crée une explosion de particules
    pub fn create_explosion(&mut self, pos: Vec2, color: Color, count: usize) {
        for _ in 0..count {
            self.particles.push(create_explosion_particle(pos, color));
        }
    }

    /// Crée un flash de bouche de canon
    pub fn create_muzzle_flash(&mut self, pos: Vec2, dir: Vec2) {
        for _ in 0..2 {
            self.particles.push(Particle {
                pos,
                vel: dir * 100.0 + vec2(rand::gen_range(-50., 50.), rand::gen_range(-50., 50.)),
                life: 0.2,
                color: SKYBLUE,
                size: 1.5,
            });
        }
    }

    /// Crée l'effet de lancement de missile
    pub fn create_missile_launch(&mut self, pos: Vec2, dir: Vec2) {
        for _ in 0..5 {
            self.particles.push(Particle {
                pos,
                vel: dir * 150.0 + vec2(rand::gen_range(-50., 50.), rand::gen_range(-50., 50.)),
                life: 0.3,
                color: ORANGE,
                size: 2.0,
            });
        }
    }

    /// Crée le flash du laser
    pub fn create_laser_flash(&mut self, pos: Vec2, dir: Vec2) {
        for _ in 0..10 {
            self.particles.push(Particle {
                pos: pos + dir * 30.0,
                vel: dir * 300.0 + vec2(rand::gen_range(-100., 100.), rand::gen_range(-100., 100.)),
                life: 0.3,
                color: Color::new(1.0, 0.2, 0.2, 1.0),
                size: 4.0,
            });
        }
    }

    /// Crée l'effet du boost de recul
    pub fn create_reverse_boost(&mut self, pos: Vec2, rot: f32) {
        for _ in 0..3 {
            let angle = rand::gen_range(rot - 0.5, rot + 0.5);
            let boost_dir = Vec2::from_angle(angle);
            self.particles.push(Particle {
                pos: pos + boost_dir * 25.0,
                vel: boost_dir * 400.0 + vec2(rand::gen_range(-100., 100.), rand::gen_range(-100., 100.)),
                life: 0.4,
                color: Color::new(0.8, 0.2, 0.8, 1.0),
                size: 3.0,
            });
        }
    }

    /// Crée un effet de texte (quand on ramasse un power-up)
    pub fn create_text_effect(&mut self, pos: Vec2, color: Color) {
        for _ in 0..20 {
            self.particles.push(Particle {
                pos,
                vel: vec2(rand::gen_range(-50., 50.), rand::gen_range(-100., -50.)),
                life: 1.0,
                color,
                size: 3.0,
            });
        }
    }

    /// Ajoute un point à la traînée du vaisseau
    pub fn add_trail_point(&mut self, pos: Vec2) {
        self.trail.push_back(TrailPoint {
            pos,
            life: 0.5,
        });
    }

    /// Traite les événements du système d'effets
    pub fn process_events(&mut self, event_queue: &mut EventQueue) {
        while let Some(event) = event_queue.pop() {
            match event {
                Event::Explosion { pos, color, count } => {
                    self.create_explosion(pos, color, count);
                }
                Event::PowerUpSpawn { pos } => {
                    self.create_text_effect(pos, GOLD);
                }
                Event::LaserFired { pos, dir } => {
                    self.create_laser_flash(pos, dir);
                }
                Event::GameOver => {
                    // Rien à faire ici, géré par les scènes
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// PARTICULE INDIVIDUELLE
// -----------------------------------------------------------------------------
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
    pub color: Color,
    pub size: f32,
}

// -----------------------------------------------------------------------------
// POINT DE TRAÎNÉE
// -----------------------------------------------------------------------------
pub struct TrailPoint {
    pub pos: Vec2,
    pub life: f32,
}

// -----------------------------------------------------------------------------
// FONCTIONS DE CRÉATION DE PARTICULES
// -----------------------------------------------------------------------------

/// Crée une particule d'explosion standard
pub fn create_explosion_particle(pos: Vec2, color: Color) -> Particle {
    Particle {
        pos,
        vel: vec2(rand::gen_range(-200., 200.), rand::gen_range(-200., 200.)),
        life: rand::gen_range(0.3, 0.6),
        color,
        size: rand::gen_range(2.0, 5.0),
    }
}
