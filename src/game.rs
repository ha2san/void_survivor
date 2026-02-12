// =============================================================================
// STRUCTURE PRINCIPALE DU JEU
// =============================================================================
//
// Ce fichier contient le coeur du jeu avec :
// - La structure Game qui agrège tous les états
// - La gestion des vagues et du spawn
// - Le système de cooldown réutilisable
// - L'ordre d'exécution des systèmes
//
// =============================================================================

use macroquad::prelude::*;
use crate::ship::Ship;
use crate::weapons::*;
use crate::enemies::*;
use crate::effects::EffectSystem;
use crate::collisions;
use crate::events::EventQueue;

// -----------------------------------------------------------------------------
// STRUCTURE PRINCIPALE - AGGRÉGATE DE TOUS LES ÉTATS
// -----------------------------------------------------------------------------
pub struct Game {
    // État de la partie
    pub state: GameState,
    pub score: i32,
    pub high_score: i32,
    pub timer: f32,
    pub combo: i32,
    pub combo_timer: Cooldown,

    // Gestion des vagues
    pub wave: i32,
    pub enemies_to_spawn: i32,
    pub enemies_killed: i32,

    // Sous-systèmes
    pub ship: Ship,
    pub world: World,
    pub weapons: WeaponSystem,
    pub effects: EffectSystem,
    pub shake: ScreenShake,
    pub event_queue: EventQueue,  // File d'événements pour découplage
}

// -----------------------------------------------------------------------------
// ÉTATS DU JEU (Machine à états)
// -----------------------------------------------------------------------------
#[derive(PartialEq)]
pub enum GameState {
    Menu,       // Écran titre
    Playing,    // Partie en cours
    Paused,     // Jeu en pause
    GameOver,   // Partie terminée
}

// -----------------------------------------------------------------------------
// MONDE - COLLECTION DE TOUTES LES ENTITÉS DYNAMIQUES
// -----------------------------------------------------------------------------
pub struct World {
    pub asteroids: Vec<Asteroid>,
    pub drones: Vec<Drone>,
    pub bullets: Vec<Bullet>,
    pub powerups: Vec<PowerUp>,
}

impl World {
    pub fn new() -> Self {
        Self {
            asteroids: Vec::new(),
            drones: Vec::new(),
            bullets: Vec::new(),
            powerups: Vec::new(),
        }
    }
}

// -----------------------------------------------------------------------------
// EFFET D'ÉCRAN - SECOUEMENT (SCREEN SHAKE)
// -----------------------------------------------------------------------------
pub struct ScreenShake {
    pub amount: f32,    // Intensité actuelle
    pub timer: f32,     // Durée restante
}

impl ScreenShake {
    pub fn new() -> Self {
        Self {
            amount: 0.0,
            timer: 0.0,
        }
    }

    /// Met à jour le screen shake et retourne le vecteur de décalage
    pub fn update(&mut self, dt: f32) -> Vec2 {
        if self.timer > 0.0 {
            self.amount = self.timer * 10.0;
            self.timer -= dt;
        } else {
            self.amount *= 0.9;
        }

        if self.amount > 0.0 {
            vec2(
                rand::gen_range(-self.amount, self.amount),
                rand::gen_range(-self.amount, self.amount),
            )
        } else {
            Vec2::ZERO
        }
    }

    /// Déclenche un secouement d'intensité donnée
    pub fn trigger(&mut self, intensity: f32) {
        self.timer = intensity;
    }
}

// -----------------------------------------------------------------------------
// SYSTÈME DE COOLDOWN RÉUTILISABLE
// -----------------------------------------------------------------------------
#[derive(Clone, Copy)]
pub struct Cooldown {
    pub time: f32,  // Temps restant
    pub max: f32,   // Temps maximum
}

impl Cooldown {
    pub fn new(max: f32) -> Self {
        Self { time: 0.0, max }
    }

    pub fn full(max: f32) -> Self {
        Self { time: max, max }
    }

    /// Décrémente le cooldown
    pub fn tick(&mut self, dt: f32) {
        self.time = (self.time - dt).max(0.0);
    }

    pub fn increase(&mut self, amount: f32) {
        self.time = (self.time + amount).min(self.max);
    }

    /// Vérifie si le cooldown est terminé
    pub fn ready(&self) -> bool {
        self.time <= 0.0
    }

    /// Réinitialise le cooldown à sa valeur maximale
    pub fn reset(&mut self) {
        self.time = self.max;
    }

    /// Définit une valeur spécifique
    pub fn set(&mut self, value: f32) {
        self.time = value.max(0.0);
    }

    /// Retourne le pourcentage de temps restant (0.0 à 1.0)
    pub fn percent(&self) -> f32 {
        self.time / self.max
    }
}

// -----------------------------------------------------------------------------
// IMPLÉMENTATION DU JEU
// -----------------------------------------------------------------------------
impl Game {
    /// Crée une nouvelle instance de jeu
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            score: 0,
            high_score: 0,
            timer: 0.0,
            combo: 0,
            combo_timer: Cooldown::new(2.0),
            wave: 1,
            enemies_to_spawn: 5,
            enemies_killed: 0,
            ship: Ship::new(),
            world: World::new(),
            weapons: WeaponSystem::new(),
            effects: EffectSystem::new(),
            shake: ScreenShake::new(),
            event_queue: EventQueue::new(),
        }
    }

    /// Réinitialise complètement le jeu pour une nouvelle partie
    pub fn reset(&mut self) {
        self.score = 0;
        self.timer = 0.0;
        self.combo = 0;
        self.combo_timer = Cooldown::new(2.0);
        self.wave = 1;
        self.enemies_to_spawn = 5;
        self.enemies_killed = 0;
        self.state = GameState::Playing;
        self.weapons.missiles.clear();
        self.ship = Ship::new();
        self.world.asteroids.clear();
        self.world.drones.clear();
        self.world.bullets.clear();
        self.world.powerups.clear();
        self.weapons = WeaponSystem::new();
        self.effects.clear();
        self.shake = ScreenShake::new();
        self.event_queue = EventQueue::new();
    }

    /// Met à jour toute la logique du jeu pour une frame
    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        self.combo_timer.tick(dt);

        if self.combo_timer.ready() {
            self.combo = 0;
        }

        // Mise à jour du vaisseau (on ignore la direction retournée)
        let _ship_dir = self.ship.update(dt);

        // Mise à jour des armes
        self.weapons.update(dt, &mut self.ship, &mut self.world, &mut self.effects, &mut self.event_queue);

        // Mise à jour des entités
        self.update_entities(dt);

        // Gestion des collisions
        self.handle_collisions();

        // Gestion des vagues
        self.handle_waves(dt);

        // Nettoyage des entités mortes
        self.cleanup();

        // Mise à jour des effets visuels
        self.effects.update(dt);

        // Mise à jour du screen shake
        self.shake.update(dt);
    }

    /// Met à jour toutes les entités dynamiques
    fn update_entities(&mut self, dt: f32) {
        // Astéroïdes
        for a in self.world.asteroids.iter_mut() {
            a.update(dt);
        }

        // Drones
        for d in self.world.drones.iter_mut() {
            d.update(dt, &self.ship.pos);

            // Tir des drones
            if let Some(bullet) = d.shoot(dt, &self.ship) {
                self.world.bullets.push(bullet);
            }
        }

        // Missiles téléguidés
        self.weapons.update_missiles(
            dt, 
            &mut self.world, 
            &mut self.effects, 
            &mut self.score, 
            &mut self.combo, 
            &mut self.combo_timer, 
            &mut self.enemies_killed, 
            &mut self.event_queue
        );

        // Laser perçant
        self.weapons.update_laser(
            dt, 
            &self.ship, 
            &mut self.world, 
            &mut self.effects, 
            &mut self.score, 
            &mut self.enemies_killed, 
            &mut self.event_queue
        );

        // Balles
        for b in self.world.bullets.iter_mut() {
            b.update(dt);
        }

        // Power-ups
        for p in self.world.powerups.iter_mut() {
            p.update(dt);
        }
    }

    /// Gère toutes les collisions entre entités
    fn handle_collisions(&mut self) {
        // On extrait les valeurs nécessaires AVANT les appels pour éviter
        // les problèmes de borrow checker (emprunts simultanés)
        let ship_pos = self.ship.pos;
        let ship_invincible = self.ship.invincible_timer.time;
        let ship_shield_active = self.ship.shield_active;

        // Collisions balles alliées vs ennemis
        collisions::handle_friendly_bullet_collisions(
            &mut self.world.bullets,
            &mut self.world.asteroids,
            &mut self.world.drones,
            &mut self.score,
            &mut self.combo,
            &mut self.combo_timer,
            &mut self.enemies_killed,
            &mut self.effects,
            &mut self.world.powerups,
            &mut self.event_queue,
        );

        // Collisions balles ennemies vs vaisseau
        collisions::handle_enemy_bullet_collisions(
            &mut self.world.bullets,
            &ship_pos,
            ship_invincible,
            ship_shield_active,
            &mut self.ship.lives,
            &mut self.ship.invincible_timer,
            &mut self.shake,
            &mut self.effects,
            &mut self.state,
            &mut self.event_queue,
        );

        // Collisions vaisseau vs astéroïdes
        collisions::handle_ship_asteroid_collisions(
            &mut self.world.asteroids,
            &ship_pos,
            ship_invincible,
            ship_shield_active,
            &mut self.ship.lives,
            &mut self.ship.invincible_timer,
            &mut self.shake,
            &mut self.score,
            &mut self.effects,
            &mut self.state,
            &mut self.event_queue,
        );

        // Collisions vaisseau vs drones
        collisions::handle_ship_drone_collisions(
            &mut self.world.drones,
            &mut self.world.asteroids,
            &ship_pos,
            ship_invincible,
            ship_shield_active,
            &mut self.ship.lives,
            &mut self.ship.invincible_timer,
            &mut self.shake,
            &mut self.effects,
            &mut self.state,
            &mut self.score,
            &mut self.enemies_killed,
            &mut self.event_queue,
        );

        // Collisions power-ups vs vaisseau
        collisions::handle_powerup_collisions(
            &mut self.world.powerups,
            &self.ship.pos,
            &mut self.ship.shield_energy,
            &mut self.ship.lives,
            &mut self.ship.slowmo_energy,
            &mut self.ship.rapid_fire_timer,
            &mut self.shake,
            &mut self.effects,
            &mut self.event_queue,
        );
    }

    /// Gère le spawn des ennemis et les changements de vague
    fn handle_waves(&mut self, _dt: f32) {
        // Vérifier si la vague est terminée
        if self.enemies_killed >= self.enemies_to_spawn {
            self.wave += 1;
            self.enemies_killed = 0;
            self.enemies_to_spawn = 5 + self.wave * 3;
            self.score += self.wave * 100;
            self.shake.trigger(0.5);

            // Effet visuel de fin de vague
            for _ in 0..50 {
                self.effects.particles.push(crate::effects::create_explosion_particle(
                        vec2(screen_width() / 2., screen_height() / 2.),
                        GOLD,
                ));
            }
        }

        // Difficulté progressive
        let time_difficulty = 1.5;
        let wave_difficulty = 1.0 + (self.wave as f32 * 0.5);
        let total_difficulty = time_difficulty * wave_difficulty;

        // Spawn aléatoire d'astéroïdes
        if rand::gen_range(0, (80.0 / total_difficulty) as i32) == 0 {
            self.world.asteroids.push(Asteroid::new(
                    vec2(rand::gen_range(0., screen_width()), -50.),
                    total_difficulty,
            ));
        }

        // Spawn aléatoire de drones (limité par la vague)
        if rand::gen_range(0, (200.0 / total_difficulty) as i32) == 0 
            && self.world.drones.len() < 5 + self.wave as usize {
                self.world.drones.push(Drone::new(
                        vec2(rand::gen_range(0., screen_width()), -20.),
                        total_difficulty,
                        self.wave,
                ));
        }
    }

    /// Nettoie les entités mortes ou hors écran
    fn cleanup(&mut self) {
        self.world.bullets.retain(|b| b.is_alive());
        self.world.drones.retain(|d| d.is_alive());
        self.world.asteroids.retain(|a| a.is_alive());
        self.world.powerups.retain(|p| p.is_alive());
        self.weapons.missiles.retain(|m| m.is_alive());
    }
}
