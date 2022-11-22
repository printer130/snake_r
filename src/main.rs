mod game;
use std::io::stdout;

use crate::game::Game;
fn main() {
    Game::new(stdout(), 30, 20).run();
}
