// =============================================================================
// MODULE RACINE - DÉCLARATION DES MODULES ET RÉEXPORTATIONS
// =============================================================================

pub mod consts;
pub mod game;
pub mod ship;
pub mod weapons;
pub mod enemies;
pub mod collisions;
pub mod effects;
pub mod rendering;
pub mod ui;
pub mod scenes;
pub mod events;

pub use consts::*;
pub use game::{Game, GameState, World, ScreenShake, Cooldown};
pub use ship::{Ship, TrailPoint};
pub use weapons::{WeaponSystem, HomingMissile, Laser};
pub use enemies::{Asteroid, Drone, DroneType, Bullet, PowerUp, PowerUpType};
pub use effects::{EffectSystem, Particle, TrailPoint as EffectTrailPoint};
pub use collisions::*;
pub use rendering::Renderer;
pub use ui::*;
pub use scenes::*;
pub use events::{Event, EventQueue};
