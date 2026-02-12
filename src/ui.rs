// =============================================================================
// INTERFACE UTILISATEUR (UI)
// =============================================================================
//
// Ce fichier gère tous les éléments d'interface :
// - HUD (score, vague, temps, barres d'état)
// - Écran titre (menu principal)
// - Écran de pause
// - Écran de game over
// - Indicateurs d'armes et de bonus
//
// =============================================================================

use macroquad::prelude::*;
use crate::consts::*;
use crate::game::Game;

pub struct UI;

impl UI {
    // -------------------------------------------------------------------------
    // HUD - PENDANT LA PARTIE
    // -------------------------------------------------------------------------

    /// Dessine l'interface complète pendant le jeu
    pub fn draw_hud(game: &Game) {
        // Score et vague
        draw_text(&format!("SCORE: {}", game.score), 20., 30., 30., WHITE);
        draw_text(&format!("VAGUE: {}", game.wave), 20., 60., 25., GRAY);
        draw_text(&format!("TEMPS: {:.1}s", game.timer), 20., 90., 20., LIGHTGRAY);

        // Barre de bouclier
        draw_rectangle(20., 110., UI_BAR_WIDTH, UI_BAR_HEIGHT, DARKGRAY);
        let shield_width = (game.ship.shield_energy.percent()) * UI_BAR_WIDTH;
        draw_rectangle(20., 110., shield_width, UI_BAR_HEIGHT, SKYBLUE);
        draw_rectangle_lines(20., 110., UI_BAR_WIDTH, UI_BAR_HEIGHT, 1., WHITE);

        // Barre de slow-motion
        draw_rectangle(20., 125., UI_BAR_WIDTH, 8., DARKGRAY);
        let slowmo_width = (game.ship.slowmo_energy.percent()) * UI_BAR_WIDTH;
        let slowmo_color = if game.ship.is_slowmo_available {
            if game.ship.slowmo_active {
                PURPLE
            } else {
                Color::new(0.6, 0.2, 1.0, 1.0)
            }
        } else {
            GRAY
        };
        draw_rectangle(20., 125., slowmo_width, 8., slowmo_color);
        draw_rectangle_lines(20., 125., UI_BAR_WIDTH, 8., 1., WHITE);
        draw_text("SLOW-MO", 180., 130., 15., slowmo_color);

        // Vies (icônes de vaisseau)
        for i in 0..game.ship.lives {
            draw_poly(30. + i as f32 * 25., 145., 3, 8., 90., RED);
        }

        // Combo
        if game.combo > 1 {
            draw_text(
                &format!("COMBO x{}!", game.combo),
                screen_width() - 150.,
                30.,
                30.,
                YELLOW,
            );
            draw_text(
                &format!("+{}", game.combo * 5),
                screen_width() - 150.,
                60.,
                20.,
                GOLD,
            );
        }

        // Indicateurs d'état (bonus actifs)
        Self::draw_status_indicators(game);
        
        // Indicateurs d'armes
        Self::draw_weapon_indicators(game);
        
        // Raccourcis clavier
        Self::draw_controls();
    }

    /// Dessine les indicateurs de bonus actifs
    fn draw_status_indicators(game: &Game) {
        let mut y_offset = 100.0;
        
        if game.ship.slowmo_active {
            draw_text(
                &format!("SLOW-MO: {:.1}s", game.ship.slowmo_energy.time),
                screen_width() - 200.,
                y_offset,
                20.,
                PURPLE,
            );
            y_offset += 25.0;
        }
        
        if !game.ship.rapid_fire_timer.ready() {
            draw_text(
                &format!("TIR RAPIDE: {:.1}s", game.ship.rapid_fire_timer.time),
                screen_width() - 200.,
                y_offset,
                20.,
                ORANGE,
            );
            y_offset += 25.0;
        }
        
        if !game.ship.reverse_boost_timer.ready() {
            draw_text(
                "RECUL BOOSTÉ!",
                screen_width() - 200.,
                y_offset,
                20.,
                Color::new(0.8, 0.2, 0.8, 1.0),
            );
            y_offset += 25.0;
        }
        
        if game.ship.is_invincible() {
            draw_text(
                &format!("INVINCIBLE: {:.1}s", game.ship.invincible_timer.time),
                screen_width() - 200.,
                y_offset,
                20.,
                GREEN,
            );
        }
    }

    /// Dessine les indicateurs d'armes (munitions, recharge)
    fn draw_weapon_indicators(game: &Game) {
        // Missiles
        draw_text(
            &format!("MISSILE: {} / {}", game.weapons.missile_ammo, MISSILE_AMMO_MAX),
            20.,
            170.,
            20.,
            ORANGE,
        );
        
        if !game.weapons.missile_cooldown.ready() {
            draw_text(
                &format!("({:.1}s)", game.weapons.missile_cooldown.time),
                150.,
                170.,
                16.,
                GRAY,
            );
        }

        // Laser
        let laser_color = if game.weapons.laser.cooldown.ready() { RED } else { GRAY };
        draw_text("LASER: ", 20., 200., 20., laser_color);
        
        if game.weapons.laser.active {
            draw_text("ACTIF", 90., 200., 20., RED);
        } else if !game.weapons.laser.cooldown.ready() {
            draw_text(
                &format!("RECHARGE {:.1}s", game.weapons.laser.cooldown.time),
                90.,
                200.,
                16.,
                GRAY,
            );
        } else {
            draw_text("PRÊT", 90., 200., 20., GREEN);
        }
    }

    /// Dessine les raccourcis clavier en bas de l'écran
    fn draw_controls() {
        draw_text(
            &format!(
                "{}:PAUSE | {}:BOUCLIER | {}:RECUL | {}:SLOW-MO | {}:MISSILE | {}:LASER",
                key_to_string(KEY_PAUSE),
                key_to_string(KEY_SHIELD),
                key_to_string(KEY_REVERSE_BOOST),
                key_to_string(KEY_SLOWMO),
                key_to_string(KEY_MISSILE),
                key_to_string(KEY_LASER)
            ),
            screen_width() / 2. - 280.,
            screen_height() - 20.,
            20.,
            GRAY,
        );
    }

    // -------------------------------------------------------------------------
    // ÉCRANS DE MENU
    // -------------------------------------------------------------------------

    /// Dessine l'écran titre (menu principal)
    pub fn draw_menu(high_score: i32) {
        Self::draw_text_centered("Void Survivor", screen_height() * 0.3, 48., YELLOW);
        Self::draw_text_centered("CONTRÔLES:", screen_height() * 0.45, 30., WHITE);
        Self::draw_text_centered(
            "← → : Tourner | ↑ : Avancer | ↓ : Reculer",
            screen_height() * 0.5,
            20.,
            GRAY,
        );
        Self::draw_text_centered(
            "ESPACE : Tirer | E : Bouclier | R : Recul rapide",
            screen_height() * 0.55,
            20.,
            GRAY,
        );
        Self::draw_text_centered(
            "S : Slow-motion (limité) | P : Pause | M : Menu",
            screen_height() * 0.6,
            20.,
            GRAY,
        );
        Self::draw_text_centered(
            "F : Missile téléguidé | L : Laser perçant",
            screen_height() * 0.65,
            20.,
            GRAY,
        );
        Self::draw_text_centered(
            &format!("MEILLEUR SCORE: {}", high_score),
            screen_height() * 0.75,
            28.,
            GOLD,
        );
        Self::draw_text_centered(
            "APPUYEZ SUR [ENTRÉE] POUR COMMENCER",
            screen_height() * 0.85,
            28.,
            GREEN,
        );
    }

    /// Dessine l'écran de pause
    pub fn draw_pause_screen(score: i32) {
        // Fond semi-transparent
        draw_rectangle(
            0.,
            0.,
            screen_width(),
            screen_height(),
            Color::new(0., 0., 0., 0.7),
        );
        
        Self::draw_text_centered("PAUSE", screen_height() * 0.4, 50., YELLOW);
        Self::draw_text_centered("P : REPRENDRE", screen_height() * 0.5, 30., WHITE);
        Self::draw_text_centered("ECHAP : MENU", screen_height() * 0.55, 30., WHITE);
        Self::draw_text_centered(
            &format!("SCORE: {}", score),
            screen_height() * 0.6,
            40.,
            GOLD,
        );
    }

    /// Dessine l'écran de game over
    pub fn draw_game_over_screen(score: i32, high_score: i32, wave: i32, flash_timer: f32) {
        // Effet de flash rouge si nécessaire
        if flash_timer > 0.0 {
            draw_rectangle(
                0.,
                0.,
                screen_width(),
                screen_height(),
                Color::new(1.0, 0.0, 0.0, flash_timer * 0.5),
            );
        }

        Self::draw_text_centered("MISSION ÉCHOUÉE", screen_height() * 0.3, 50., RED);
        Self::draw_text_centered(
            &format!("SCORE FINAL : {}", score),
            screen_height() * 0.4,
            40.,
            WHITE,
        );

        // Message spécial pour nouveau record
        if score > high_score {
            Self::draw_text_centered("NOUVEAU RECORD !", screen_height() * 0.45, 35., GOLD);
        }

        Self::draw_text_centered(
            &format!("VAGUE ATTEINTE : {}", wave),
            screen_height() * 0.5,
            30.,
            GRAY,
        );
        Self::draw_text_centered(
            "ENTRÉE POUR RECOMMENCER",
            screen_height() * 0.6,
            25.,
            GREEN,
        );
        Self::draw_text_centered(
            "ECHAP POUR MENU",
            screen_height() * 0.65,
            20.,
            GRAY,
        );
    }

    // -------------------------------------------------------------------------
    // FONCTIONS UTILITAIRES
    // -------------------------------------------------------------------------

    /// Dessine du texte centré horizontalement
    fn draw_text_centered(text: &str, y: f32, size: f32, color: Color) {
        let dims = measure_text(text, None, size as u16, 1.0);
        draw_text(
            text,
            screen_width() / 2. - dims.width / 2.,
            y,
            size,
            color,
        );
    }
}

// -----------------------------------------------------------------------------
// CONVERSION TOUCHE -> TEXTE POUR L'AFFICHAGE
// -----------------------------------------------------------------------------

/// Convertit un code de touche en chaîne de caractères lisible
pub fn key_to_string(key: KeyCode) -> &'static str {
    match key {
        KEY_UP => "W",
        KEY_DOWN => "S",
        KEY_LEFT => "A",
        KEY_RIGHT => "D",
        KEY_SHOOT => "ESPACE",
        KEY_SHIELD => "I",
        KEY_REVERSE_BOOST => "K",
        KEY_SLOWMO => "H",
        KEY_MISSILE => "J",
        KEY_LASER => "L",
        KEY_PAUSE => "P",
        KEY_MENU => "ECHAP",
        KEY_START => "ENTRÉE",
        _ => "?",
    }
}
