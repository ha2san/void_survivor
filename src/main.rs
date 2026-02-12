// =============================================================================
// POINT D'ENTRÉE PRINCIPAL
// =============================================================================
//
// Ce fichier est le point d'entrée du jeu. Il initialise la fenêtre,
// crée le jeu et la scène initiale, puis lance la boucle principale.
//
// Architecture :
// - Game : contient tout l'état du jeu
// - Scene : gère l'état actuel (menu, jeu, pause, game over)
// - EventQueue : file d'événements pour découplage
//
// =============================================================================

use macroquad::prelude::*;
use void_survivor::*;
use void_survivor::game::Game;
use void_survivor::events::EventQueue;

#[macroquad::main("Void Survivor")]
async fn main() {
    // Configuration initiale
    let mut game = Game::new();
    let mut event_queue = EventQueue::new();
    let mut current_scene: Box<dyn Scene> = Box::new(MenuScene::new());

    // Boucle principale du jeu
    loop {
        // Temps écoulé depuis la dernière frame
        let dt_raw = get_frame_time();
        
        let dt = dt_raw * game.ship.get_slow_time_factor();
        
        // Effacer l'écran (noir profond)
        clear_background(color_u8!(10, 10, 30, 255));

        // Mise à jour de la scène courante
        // Si elle retourne une nouvelle scène, on change
        let transition = current_scene.update(&mut game, &mut event_queue, dt);
        
        if let Some(new_scene) = transition {
            current_scene = new_scene;
        }

        // Dessin de la scène courante
        current_scene.draw(&mut game);

        // Attendre la prochaine frame
        next_frame().await;
    }
}
