// =============================================================================
// MOTEUR DE RENDU
// =============================================================================
//
// Ce fichier gère tout l'affichage graphique :
// - Étoiles de fond
// - Vaisseau et sa traînée
// - Ennemis (astéroïdes, drones)
// - Armes (balles, missiles, laser)
// - Effets visuels (particules)
// - Interface utilisateur (UI)
//
// =============================================================================

use macroquad::prelude::*;
use std::collections::VecDeque;
use crate::consts::*;
use crate::game::*;
use crate::ship::Ship;
use crate::enemies::*;
use crate::weapons::*;
use crate::effects;
use crate::ui::*;

pub struct Renderer;

impl Renderer {
    /// Crée un nouveau renderer
    pub fn new() -> Self {
        Self
    }

    // -------------------------------------------------------------------------
    // FOND
    // -------------------------------------------------------------------------

    /// Dessine les étoiles animées en arrière-plan
    pub fn draw_stars(&self) {
        let time = get_time() as f32;
        for i in 0..150 {
            let i_f32 = i as f32;
            let x = (time * 0.3 + i_f32 * 0.07).sin() * 30.0 + (i_f32 * 13.7).sin() * screen_width();
            let y = (i_f32 * 7.3 + time * 0.2).sin() * screen_height();
            let size = ((i % 4) + 1) as f32;
            let brightness = (80.0 + (175.0 * ((time * 2.0 + i_f32 * 0.3).sin() * 0.5 + 0.5))) as u8;
            let alpha = 150 + (brightness / 10) as u8;
            draw_circle(
                x as f32,
                y as f32,
                size,
                color_u8!(brightness, brightness, brightness, alpha),
            );
        }
    }

    // -------------------------------------------------------------------------
    // ENTITÉS
    // -------------------------------------------------------------------------

    /// Dessine la scène de jeu complète
    pub fn draw_game(&self, game: &Game, shake_vec: Vec2) {
        self.draw_stars();
        self.draw_ship_trail(&game.effects.trail, &shake_vec);
        self.draw_particles(&game.effects.particles);
        self.draw_asteroids(&game.world.asteroids);
        self.draw_drones(&game.world.drones, game.wave);
        self.draw_bullets(&game.world.bullets, &game.ship.rapid_fire_timer);
        self.draw_missiles(&game.weapons.missiles);
        self.draw_laser(&game.weapons.laser, &shake_vec);
        self.draw_powerups(&game.world.powerups);
        self.draw_ship(&game.ship, &shake_vec);
        UI::draw_hud(game);
    }

    /// Dessine la traînée du vaisseau
    fn draw_ship_trail(&self, trail: &VecDeque<effects::TrailPoint>, shake_vec: &Vec2) {
        for (i, tp) in trail.iter().enumerate() {
            let alpha = tp.life / 0.5;
            let size = 3.0 * (i as f32 / trail.len() as f32);
            draw_circle(
                tp.pos.x + shake_vec.x,
                tp.pos.y + shake_vec.y,
                size,
                Color::new(0.2, 0.6, 1.0, alpha * 0.3),
            );
        }
    }

    /// Dessine les particules (effets visuels)
    pub fn draw_particles(&self, particles: &Vec<effects::Particle>) {
        for p in particles {
            let alpha = p.life / 0.4;
            draw_circle(
                p.pos.x,
                p.pos.y,
                p.size,
                Color::new(p.color.r, p.color.g, p.color.b, alpha),
            );
        }
    }

    /// Dessine les astéroïdes
    fn draw_asteroids(&self, asteroids: &Vec<Asteroid>) {
        for a in asteroids {
            draw_circle_lines(a.pos.x, a.pos.y, a.radius, 2., WHITE);
            let detail_pos = a.pos + Vec2::from_angle(a.rotation) * (a.radius * 0.7);
            draw_circle(detail_pos.x, detail_pos.y, a.radius * 0.2, DARKGRAY);
        }
    }

    /// Dessine les drones
    fn draw_drones(&self, drones: &Vec<Drone>, wave: i32) {
        for d in drones {
            let c = match d.kind {
                DroneType::Kamikaze => ORANGE,
                DroneType::Sniper => RED,
                DroneType::Bomber => PURPLE,
            };
            
            draw_rectangle(d.pos.x - 10., d.pos.y - 10., 20., 20., c);
            draw_rectangle_lines(d.pos.x - 10., d.pos.y - 10., 20., 20., 2., WHITE);

            // Barre de vie
            let max_hp = 2.0 + (wave / 3) as f32;
            let health_width = 20.0 * (d.hp as f32 / max_hp);
            draw_rectangle(d.pos.x - 10., d.pos.y - 15., health_width, 3., GREEN);
            
            // Indicateur de menace pour les drones qui tirent
            if d.kind == DroneType::Sniper || d.kind == DroneType::Bomber {
                self.draw_threat_indicator(d.pos);
            }
        }
    }

    /// Dessine un indicateur de menace autour d'un drone
    fn draw_threat_indicator(&self, pos: Vec2) {
        let time = get_time() as f32;
        let radius = 22.0 + (time * 6.0).sin() * 3.0;
        draw_circle_lines(pos.x, pos.y, radius, 2.5, RED);
        
        let text = "MENACE";
        let dims = measure_text(text, None, 20, 1.0);
        draw_text(
            text,
            pos.x - dims.width / 2.0,
            pos.y - radius - 10.0,
            20.0,
            RED,
        );
    }

    /// Dessine les balles
    fn draw_bullets(&self, bullets: &Vec<Bullet>, rapid_fire_timer: &Cooldown) {
        for b in bullets {
            let color = if b.enemy {
                if b.size > 5.0 {
                    RED
                } else {
                    Color::new(1.0, 0.5, 0.0, 1.0)
                }
            } else {
                if rapid_fire_timer.time > 0.0 {
                    Color::new(0.0, 1.0, 1.0, 1.0)
                } else {
                    SKYBLUE
                }
            };
            draw_circle(b.pos.x, b.pos.y, b.size, color);
        }
    }

    /// Dessine les missiles téléguidés
    fn draw_missiles(&self, missiles: &Vec<HomingMissile>) {
        for m in missiles {
            // Traînée du missile
            let trail_len = 5;
            for i in 0..trail_len {
                let t = i as f32 / trail_len as f32;
                let pos_trail = m.pos - m.vel * t * 0.05;
                draw_circle(
                    pos_trail.x,
                    pos_trail.y,
                    3.0 * (1.0 - t),
                    Color::new(1.0, 0.5, 0.0, 0.5 - t * 0.4),
                );
            }
            draw_circle(m.pos.x, m.pos.y, 4.0, ORANGE);
            draw_circle(m.pos.x, m.pos.y, 6.0, Color::new(1.0, 0.6, 0.0, 0.4));
        }
    }

    /// Dessine le laser
    fn draw_laser(&self, laser: &Laser, shake_vec: &Vec2) {
        if laser.active {
            let beam_start = laser.start_pos + *shake_vec;
            let beam_dir = laser.direction;
            let beam_end = beam_start + beam_dir * LASER_RANGE;

            draw_line(beam_start.x, beam_start.y, beam_end.x, beam_end.y, 3.0, RED);
            draw_line(
                beam_start.x,
                beam_start.y,
                beam_end.x,
                beam_end.y,
                8.0,
                Color::new(1.0, 0.0, 0.0, 0.2),
            );

            // Étincelles sur le trajet
            for _ in 0..3 {
                let t = rand::gen_range(0.0, 1.0);
                let p = beam_start + beam_dir * t * LASER_RANGE;
                draw_circle(p.x, p.y, 2.0, RED);
            }
        }
    }

    /// Dessine les power-ups
    fn draw_powerups(&self, powerups: &Vec<PowerUp>) {
        for p in powerups {
            let time = get_time() as f32;
            draw_poly(p.pos.x, p.pos.y, 6, 10.0, time * 100.0, p.color());
            draw_poly_lines(p.pos.x, p.pos.y, 6, 10.0, time * 100.0, 2.0, WHITE);
        }
    }

    /// Dessine le vaisseau du joueur
    fn draw_ship(&self, ship: &Ship, shake_vec: &Vec2) {
        // Couleur selon l'état (invincible, bouclier, etc.)
        let mut ship_color = if ship.is_invincible() && (ship.invincible_timer.time * 10.0) as i32 % 2 == 0 {
            Color::new(1.0, 1.0, 1.0, 0.5)  // Clignotement
        } else if ship.shield_active {
            SKYBLUE  // Bouclier activé
        } else {
            BLUE     // Normal
        };

        // Effet de surbrillance pour le boost de recul
        if !ship.reverse_boost_timer.ready() {
            let pulse = (ship.reverse_boost_timer.time * 10.0).sin().abs();
            ship_color = Color::new(
                ship_color.r * 0.7 + 0.3 * pulse,
                ship_color.g * 0.7,
                ship_color.b * 0.7 + 0.3 * pulse,
                ship_color.a,
            );
        }

        let pos = ship.pos + *shake_vec;
        let rot_deg = ship.rot.to_degrees();

        draw_poly(pos.x, pos.y, 3, SHIP_RADIUS, rot_deg, ship_color);
        draw_poly_lines(pos.x, pos.y, 3, SHIP_RADIUS, rot_deg, 2.0, WHITE);

        // Effet de moteur
        let rear_dir = -ship.dir;
        let rear_pos = ship.pos + rear_dir * 10.0;
        draw_line(
            rear_pos.x,
            rear_pos.y,
            rear_pos.x + rear_dir.x * 8.0,
            rear_pos.y + rear_dir.y * 8.0,
            2.0,
            Color::new(1.0, 1.0, 1.0, 0.7),
        );

        // Bouclier visuel
        if ship.shield_active {
            draw_circle_lines(ship.pos.x, ship.pos.y, 28., 2., SKYBLUE);
            draw_circle_lines(ship.pos.x, ship.pos.y, 25., 1., Color::new(0.5, 0.8, 1.0, 0.5));
        }
    }
}
