use crate::game::Game;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct GameManager {
    games: BTreeMap<Uuid, Arc<Mutex<Game>>>,
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager { games: BTreeMap::new() }
    }

    /// Create and return a new game instance.
    /// The created game will have a UUID associated with it.
    pub fn new_game(&mut self) -> Arc<Mutex<Game>> {
        let id = Uuid::new_v4();
        let game = Game::new_with_id(Some(id.to_string()));
        let game_ref = Arc::new(Mutex::new(game));

        self.games.insert(id, game_ref.clone());
        game_ref
    }

    /// Get the list of all games.
    pub fn get_all_games(&self) -> Vec<Arc<Mutex<Game>>> {
        self.games.values().cloned().collect()
    }

    pub fn get_game(&self, id: Uuid) -> Option<Arc<Mutex<Game>>> {
        self.games.get(&id).cloned()
    }

    pub fn delete_game(&mut self, id: Uuid) {
        self.games.remove(&id);
    }
}

impl Default for GameManager {
    fn default() -> Self {
        GameManager::new()
    }
}
