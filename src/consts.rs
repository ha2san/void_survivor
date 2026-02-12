use macroquad::prelude::*;

// =============================================================================
// CONSTANTES DU JEU - TOUTES LES VALEURS CONFIGURABLES CENTRALISÉES
// =============================================================================

// -----------------------------------------------------------------------------
// CONTRÔLES (TOUCHES) - MODIFIABLES ICI
// -----------------------------------------------------------------------------
pub const KEY_SHOOT: KeyCode = KeyCode::Space;           // Tirer
pub const KEY_SHIELD: KeyCode = KeyCode::E;             // Activer bouclier
pub const KEY_REVERSE_BOOST: KeyCode = KeyCode::R;      // Recul boosté
pub const KEY_SLOWMO: KeyCode = KeyCode::S;             // Slow-motion
pub const KEY_MISSILE: KeyCode = KeyCode::F;            // Missile téléguidé
pub const KEY_LASER: KeyCode = KeyCode::L;              // Laser perçant
pub const KEY_PAUSE: KeyCode = KeyCode::P;              // Mettre en pause
pub const KEY_MENU: KeyCode = KeyCode::Escape;          // Retour au menu
pub const KEY_UP: KeyCode = KeyCode::Up;                // Avancer
pub const KEY_DOWN: KeyCode = KeyCode::Down;            // Reculer
pub const KEY_LEFT: KeyCode = KeyCode::Left;            // Tourner gauche
pub const KEY_RIGHT: KeyCode = KeyCode::Right;          // Tourner droite
pub const KEY_START: KeyCode = KeyCode::Enter;          // Commencer la partie

// -----------------------------------------------------------------------------
// CONSTANTES DE GAMEPLAY - ÉQUILIBRAGE
// -----------------------------------------------------------------------------
pub const BULLET_SPEED: f32 = 600.0;                    // Vitesse des tirs
pub const MAX_SHIELD_TIME: f32 = 5.0;                   // Durée max bouclier (secondes)
pub const MAX_SLOWMO_TIME: f32 = 3.0;                   // Durée max slow-mo (secondes)
pub const PLAYER_LIVES: i32 = 3;                        // Vies initiales
pub const PLAYER_INVINCIBLE_TIME: f32 = 2.0;            // Invincibilité après dégât
pub const POWERUP_SPAWN_CHANCE: f32 = 0.3;              // Chance de spawn power-up (30%)

// -----------------------------------------------------------------------------
// ARMES - MISSILES TÉLÉGUIDÉS
// -----------------------------------------------------------------------------
pub const MISSILE_AMMO_MAX: i32 = 5;                    // Munitions max
pub const MISSILE_COOLDOWN_MAX: f32 = 0.5;              // Délai entre deux tirs
pub const MISSILE_LIFE: f32 = 3.0;                      // Durée de vie (secondes)
pub const MISSILE_SPEED: f32 = 400.0;                   // Vitesse de déplacement
pub const MISSILE_TURN_SPEED: f32 = 3.0;                // Agilité du virage

// -----------------------------------------------------------------------------
// ARMES - LASER PERÇANT
// -----------------------------------------------------------------------------
pub const LASER_DURATION: f32 = 1.5;                    // Durée du tir (secondes)
pub const LASER_COOLDOWN: f32 = 2.0;                    // Temps de recharge
pub const LASER_DAMAGE: i32 = 2;                        // Dégâts par impact
pub const LASER_MAX_PENETRATION: i32 = 5;               // Ennemis traversés max

// -----------------------------------------------------------------------------
// CONSTANTES PHYSIQUES ET DE COLLISION
// -----------------------------------------------------------------------------
pub const SCREEN_WIDTH: f32 = 800.0;                    // Largeur écran (par défaut)
pub const SCREEN_HEIGHT: f32 = 600.0;                   // Hauteur écran (par défaut)
pub const LASER_RANGE: f32 = 800.0;                     // Portée du laser
pub const SHIP_RADIUS: f32 = 15.0;                      // Rayon de collision vaisseau
pub const DRONE_COLLISION_RADIUS: f32 = 20.0;           // Rayon collision drones
pub const BULLET_COLLISION_RADIUS: f32 = 15.0;          // Rayon collision balles
pub const POWERUP_COLLISION_RADIUS: f32 = 20.0;         // Rayon collision power-ups

// -----------------------------------------------------------------------------
// CONSTANTES UI - POSITIONNEMENT
// -----------------------------------------------------------------------------
pub const UI_MARGIN: f32 = 20.0;                        // Marge générale
pub const UI_BAR_WIDTH: f32 = 150.0;                    // Largeur barres d'état
pub const UI_BAR_HEIGHT: f32 = 10.0;                    // Hauteur barres d'état
