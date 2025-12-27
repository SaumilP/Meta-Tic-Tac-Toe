use dashmap::DashMap;
use uuid::Uuid;

use crate::game::Game;

#[derive(Default)]
pub struct AppState {
    pub games: DashMap<Uuid, Game>,
}
