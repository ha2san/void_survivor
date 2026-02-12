// =============================================================================
// GESTION DES SCÈNES (MACHINE À ÉTATS)
// =============================================================================
//
// Ce fichier implémente le pattern "Scene" pour gérer les différents états du jeu :
// - MenuScene : écran titre
// - GameScene : partie en cours
// - PauseScene : jeu en pause
// - GameOverScene : fin de partie
//
// =============================================================================

use macroquad::prelude::*;
use crate::consts::*;
use crate::game::*;
use crate::rendering::Renderer;
use crate::ui::UI;
use crate::events::EventQueue;

// -----------------------------------------------------------------------------
// TRAIT SCENE - INTERFACE COMMUNE À TOUTES LES SCÈNES
// -----------------------------------------------------------------------------
pub trait Scene {
    /// Met à jour la logique de la scène
    /// Retourne Option<Box<dyn Scene>> pour changer de scène
    fn update(&mut self, game: &mut Game, events: &mut EventQueue, dt: f32) -> Option<Box<dyn Scene>>;
    
    /// Dessine la scène
    fn draw(&self, game: &mut Game);  // &mut Game car on modifie le screen shake
}

// -----------------------------------------------------------------------------
// SCÈNE 1 : MENU PRINCIPAL
// -----------------------------------------------------------------------------
pub struct MenuScene;

impl MenuScene {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for MenuScene {
    fn update(&mut self, game: &mut Game, _events: &mut EventQueue, _dt: f32) -> Option<Box<dyn Scene>> {
        // Appuyer sur ENTREE pour commencer
        if is_key_pressed(KEY_START) {
            game.reset();
            return Some(Box::new(GameScene::new()));
        }
        None
    }

    fn draw(&self, game: &mut Game) {
        UI::draw_menu(game.high_score);
    }
}

// -----------------------------------------------------------------------------
// SCÈNE 2 : PARTIE EN COURS
// -----------------------------------------------------------------------------
pub struct GameScene {
    renderer: Renderer,
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, game: &mut Game, events: &mut EventQueue, dt: f32) -> Option<Box<dyn Scene>> {
        // Vérifier les entrées de changement de scène
        if is_key_pressed(KEY_PAUSE) {
            return Some(Box::new(PauseScene::new()));
        }
        if is_key_pressed(KEY_MENU) {
            return Some(Box::new(MenuScene::new()));
        }

        // Mise à jour du jeu
        game.update(dt);
        events.process(game);
        
        // Vérifier si la partie est terminée
        if game.state == GameState::GameOver {
            if game.score > game.high_score {
                game.high_score = game.score;
            }
            return Some(Box::new(GameOverScene::new()));
        }

        None
    }

    fn draw(&self, game: &mut Game) {
        // Récupérer le vecteur de screen shake
        let shake_vec = game.shake.update(0.0);  // dt déjà appliqué dans update
        self.renderer.draw_game(game, shake_vec);
    }
}

// -----------------------------------------------------------------------------
// SCÈNE 3 : PAUSE
// -----------------------------------------------------------------------------
pub struct PauseScene;

impl PauseScene {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for PauseScene {
    fn update(&mut self, _game: &mut Game, _events: &mut EventQueue, _dt: f32) -> Option<Box<dyn Scene>> {
        // Reprendre la partie
        if is_key_pressed(KEY_PAUSE) {
            return Some(Box::new(GameScene::new()));
        }
        // Retour au menu
        if is_key_pressed(KEY_MENU) {
            return Some(Box::new(MenuScene::new()));
        }
        None
    }

    fn draw(&self, game: &mut Game) {
        // Dessiner le jeu en arrière-plan (figé)
        let renderer = Renderer::new();
        renderer.draw_game(game, Vec2::ZERO);
        
        // Dessiner l'écran de pause par-dessus
        UI::draw_pause_screen(game.score);
    }
}

// -----------------------------------------------------------------------------
// SCÈNE 4 : GAME OVER
// -----------------------------------------------------------------------------
pub struct GameOverScene;

impl GameOverScene {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for GameOverScene {
    fn update(&mut self, game: &mut Game, events: &mut EventQueue, dt: f32) -> Option<Box<dyn Scene>> {
        // Mise à jour des effets visuels (particules d'explosion)
        game.effects.update(dt);
        events.process(game);
        
        // Recommencer
        if is_key_pressed(KEY_START) {
            game.reset();
            return Some(Box::new(GameScene::new()));
        }
        // Retour au menu
        if is_key_pressed(KEY_MENU) {
            return Some(Box::new(MenuScene::new()));
        }
        None
    }

    fn draw(&self, game: &mut Game) {
        // Dessiner le jeu figé en arrière-plan
        let renderer = Renderer::new();
        renderer.draw_game(game, Vec2::ZERO);
        
        // Dessiner l'écran de game over
        UI::draw_game_over_screen(game.score, game.high_score, game.wave, 0.0);
        
        // Dessiner les particules par-dessus (explosions)
        renderer.draw_particles(&game.effects.particles);
    }
}
