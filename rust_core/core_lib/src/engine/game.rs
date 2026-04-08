pub struct Game {
    is_initialized: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            is_initialized: false
         }
    }

    pub fn is_game_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn start(&mut self) {
        self.is_initialized = true;
    }
}