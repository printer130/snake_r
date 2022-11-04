mod game;
use std::io::stdout;

use crate::game::Game;
fn main() {
    Game::new(stdout(), 10, 10).run();
}