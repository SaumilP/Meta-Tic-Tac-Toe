use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    JoinRoom { room_id: Uuid },
    MakeMove { x: usize, y: usize },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Joined {
        player_id: Uuid,
        symbol: Cell,
    },
    GameState(GameStateDto),
    InvalidMove(String),
    GameOver {
        winner: Option<Cell>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameStateDto {
    pub board: Vec<Vec<Cell>>,
    pub current_turn: Cell,
}
