use core_lib::engine::{Game};

fn main() {
    let mut game: Game = Game::new();
    game.start();

    println!("Is game initialized: {}", game.is_game_initialized());
}
