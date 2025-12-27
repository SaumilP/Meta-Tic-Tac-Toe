pub mod win_check;
pub mod state;

use uuid::Uuid;
use shared::protocol::{Cell, GameStateDto};

#[derive(Clone)]
pub struct Game {
    pub id: Uuid,
    pub board: Vec<Vec<Cell>>,
    pub players: [Uuid; 2],
    pub turn: usize,
}

impl Game {
    pub fn new(size: usize, p1: Uuid, p2: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            board: vec![vec![Cell::Empty; size]; size],
            players: [p1, p2],
            turn: 0,
        }
    }

    pub fn current_cell(&self) -> Cell {
        if self.turn == 0 { Cell::X } else { Cell::O }
    }

    pub fn to_dto(&self) -> GameStateDto {
        GameStateDto {
            board: self.board.clone(),
            current_turn: self.current_cell(),
        }
    }
}
