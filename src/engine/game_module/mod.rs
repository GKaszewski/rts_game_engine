use comfy::*;

use crate::engine::pathfinding_module::*;

use super::{camera_module::RtsCamera, level_module::Map};

#[derive(Debug, Clone, Copy)]
pub enum PlaceMode {
    Start,
    End,
    Wall,
    Walkable,
}

pub struct GameState {
    pub board: PathfindingTerrain,
    pub start: Option<Node>,
    pub end: Option<Node>,
    pub place_mode: PlaceMode,
    pub level: Map,
    pub rts_camera: RtsCamera,
    pub draw_pathfinding: bool,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            board: PathfindingTerrain::new(47, 36, None),
            start: None,
            end: None,
            place_mode: PlaceMode::Wall,
            level: Map::new(),
            rts_camera: RtsCamera::new(),
            draw_pathfinding: false,
        }
    }

    pub fn set_start(&mut self, node: Node) {
        self.start = Some(node);
    }

    pub fn set_end(&mut self, node: Node) {
        self.end = Some(node);
    }

    pub fn set_board_based_on_tilemap(&mut self) {
        self.board.set_pathfinding_based_on_tilemap(&self.level.map);
    }
}
