// =============================================================================
// SYSTÈME D'ÉVÉNEMENTS
// =============================================================================
//
// Ce fichier implémente un système de découplage entre la logique de jeu
// et les effets visuels. Les événements sont émis par le gameplay et
// consommés par le système d'effets.
//
// Avantages :
// - Réduction du couplage
// - Code plus propre et modulaire
// - Facilite l'ajout de nouveaux effets
//
// =============================================================================

use macroquad::prelude::*;
use std::collections::VecDeque;
use crate::game::Game;

// -----------------------------------------------------------------------------
// TYPES D'ÉVÉNEMENTS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Event {
    /// Explosion à une position
    Explosion {
        pos: Vec2,
        color: Color,
        count: usize,
    },
    
    /// Spawn d'un power-up
    PowerUpSpawn {
        pos: Vec2,
    },
    
    /// Tir laser
    LaserFired {
        pos: Vec2,
        dir: Vec2,
    },
    
    /// Partie terminée
    GameOver,
}

// -----------------------------------------------------------------------------
// FILE D'ÉVÉNEMENTS
// -----------------------------------------------------------------------------

pub struct EventQueue {
    queue: VecDeque<Event>,
}

impl EventQueue {
    /// Crée une nouvelle file d'événements vide
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Ajoute un événement à la file
    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    /// Récupère et retire le prochain événement de la file
    pub fn pop(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }

    /// Traite tous les événements en attente
    pub fn process(&mut self, game: &mut Game) {
        game.effects.process_events(self);
    }

    /// Vide la file d'événements
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Retourne le nombre d'événements en attente
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Vérifie si la file est vide
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}
