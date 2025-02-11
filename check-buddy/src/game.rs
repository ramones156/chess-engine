use crate::uci_move::UciMove;
use crate::*;
use std::collections::HashMap;

pub struct Game {
    pub info: HashMap<String, String>,
    pub board_map: BoardMap,
    pub historical_moves: Vec<UciMove>,
    pub result: Option<bool>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            info: HashMap::with_capacity(12),
            board_map: BoardMap::starting(),
            historical_moves: vec![],
            result: None,
        }
    }
}
