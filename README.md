
---

# Void Survivor

A fast-paced, arcade-style space shooter developed in Rust using the [Macroquad](https://macroquad.rs/) game engine. Defeat waves of increasingly difficult enemies, manage your resources, and survive the void.

**Play now in your browser:** [https://ha2san.github.io/void_survivor/](https://ha2san.github.io/void_survivor/)

## Features

* **Advanced Movement:** Physics-based player ship with rotation, thrust, and a tactical reverse boost.
* **Resource Management:** Balance shield energy, slow-motion reserves, and weapon cooldowns.
* **Weapon Systems:** - Standard rapid-fire projectiles.
* Auto-regenerating homing missiles.
* High-impact piercing laser.


* **Diverse Enemies:** - Destructible asteroids that break into smaller fragments.
* Three specialized drone types: Kamikaze, Sniper, and Bomber.


* **Visuals and Juice:** Screen shake, particle-based explosions, muzzle flashes, and procedural trails.
* **Progression:** Dynamic wave system with a combo-based scoring mechanic.
* **Power-ups:** Collectable drops for Shield Refill, Extra Lives, Slow-mo Refill, and Rapid Fire.

## Controls

| Action | Key |
| --- | --- |
| **Movement** | W (Thrust), S (Brake), A/D (Turn) |
| **Primary Fire** | Space |
| **Shield** | I (Hold) |
| **Reverse Boost** | K |
| **Slow Motion** | H (Hold) |
| **Homing Missile** | J |
| **Piercing Laser** | L |
| **System** | P (Pause), Esc (Menu), Enter (Start) |

*Keybindings can be customized within `src/consts.rs`.*

## Game Mechanics

* **Scoring:** Maintain a high combo multiplier by defeating enemies in quick succession.
* **Defense:** The shield absorbs all incoming damage but prevents energy regeneration while active.
* **Tactics:** Use Slow-motion to navigate dense asteroid fields or dodge sniper fire.
* **Pickups:**
* **Green:** Extra Life
* **Blue:** Shield Refill
* **Purple:** Slow-mo Refill
* **Orange:** Temporary Rapid Fire



## Development

### Prerequisites

* Rust (Stable) and Cargo: [https://rustup.rs/](https://rustup.rs/)
* Target for WebAssembly: `rustup target add wasm32-unknown-unknown`

### Native Build

To run the game on your desktop:

```bash
cargo run --release

```

### WebAssembly Build

To compile for the web and run locally:

1. **Build the binary:**
```bash
cargo build --release --target wasm32-unknown-unknown

```


2. **Prepare the deployment:**
Ensure your `index.html` is pointing to the correct `.wasm` path. Typically, you move the binary to your root or a `web/` folder:
```bash
cp target/wasm32-unknown-unknown/release/void_survivor.wasm .

```


3. **Serve the files:**
Use a local HTTP server to avoid CORS issues:
```bash
# Using Python
python -m http.server 8000
# Using basic-http-server
basic-http-server .

```



## Project Architecture

The project follows a modular structure for easy maintainability:

* `ship.rs` / `weapons.rs`: Core player mechanics and combat systems.
* `enemies.rs`: AI behavior for drones and asteroid physics.
* `collisions.rs`: Optimized collision detection logic.
* `events.rs`: An event queue system used to decouple gameplay logic from visual effects.
* `rendering.rs` / `ui.rs`: Drawing routines and HUD management.
* `scenes.rs`: State management for menus, gameplay, and game-over screens.

## Screenshot 
![App Screenshot](screenshot.png)

## License

This project is available for educational and personal use. Feel free to modify and adapt the code for your own learning journey in Rust game development.

---

*Void Survivor – High-performance Rust gaming.*

---

