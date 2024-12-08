#[cfg(feature = "buddy-alloc")]
mod alloc;
mod fruit;
mod game;
mod game_status;
mod palette;
mod point;
mod snake;
mod wasm4;

use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

const RANDOM_SEED: u64 = 231;

lazy_static! {
    static ref SNAKE_GAME: Mutex<Game> = Mutex::new(Game::new(RANDOM_SEED));
}

#[no_mangle]
fn start() {
    palette::set_palette();
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game_state").update();
}
