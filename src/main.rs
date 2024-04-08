use comfy::*;
mod engine;

use engine::pathfinding_module::*;

simple_game!("RTS Game Engine", GameState, config, setup, update);

#[derive(Debug, Clone, Copy)]
enum PlaceMode {
    Start,
    End,
    Wall,
    Walkable,
}

struct GameState {
    board: PathfindingTerrain,
    start: Option<Node>,
    end: Option<Node>,
    place_mode: PlaceMode,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            board: PathfindingTerrain::new(10, 10, vec2(-10.0, -5.0)),
            start: None,
            end: None,
            place_mode: PlaceMode::Wall,
        }
    }

    pub fn set_start(&mut self, node: Node) {
        self.start = Some(node);
    }

    pub fn set_end(&mut self, node: Node) {
        self.end = Some(node);
    }
}

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(600 * 16 / 9, 600),
        ..config
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn place(_state: &mut GameState, x: i32, y: i32) {
    match _state.place_mode {
        PlaceMode::Start => {
            let node = Node {
                x,
                y,
                node_type: Some(NodeType::Start),
            };
            for y in 0.._state.board.height {
                for x in 0.._state.board.width {
                    let node = _state.board.get_node_by_position(x, y);
                    if let Some(node) = node {
                        if node.node_type == Some(NodeType::Start) {
                            _state.board.set_node_type(x, y, NodeType::Walkable);
                        }
                    }
                }
            }

            _state.set_start(node);
            _state.board.set_node_type(x, y, NodeType::Start);
        }
        PlaceMode::End => {
            for y in 0.._state.board.height {
                for x in 0.._state.board.width {
                    let node = _state.board.get_node_by_position(x, y);
                    if let Some(node) = node {
                        if node.node_type == Some(NodeType::End) {
                            _state.board.set_node_type(x, y, NodeType::Walkable);
                        }
                    }
                }
            }

            _state.set_end(Node {
                x,
                y,
                node_type: Some(NodeType::End),
            });
            _state.board.set_node_type(x, y, NodeType::End);
        }
        PlaceMode::Wall => {
            _state.board.set_node_type(x, y, NodeType::Unwalkable);
        }
        PlaceMode::Walkable => {
            _state.board.set_node_type(x, y, NodeType::Walkable);
        }
    }
}

fn update(_state: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Num1) {
        _state.place_mode = PlaceMode::Start;
    }
    if is_key_pressed(KeyCode::Num2) {
        _state.place_mode = PlaceMode::End;
    }
    if is_key_pressed(KeyCode::Num3) {
        _state.place_mode = PlaceMode::Wall;
    }
    if is_key_pressed(KeyCode::Num4) {
        _state.place_mode = PlaceMode::Walkable;
    }

    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_world();
        let node = _state.board.get_node_by_position(
            (mouse_pos.x - _state.board.offset.x) as i32,
            (mouse_pos.y - _state.board.offset.y) as i32,
        );
        if let Some(node) = node {
            place(_state, node.x, node.y);
        }
    }

    if is_key_pressed(KeyCode::Space) {}

    _state.board.clear_path();

    if let Some(start) = _state.start {
        if let Some(end) = _state.end {
            let path = _state.board.get_path(start, end);
            if let Some(path) = path {
                for node in path {
                    if node.node_type == Some(NodeType::Start)
                        || node.node_type == Some(NodeType::End)
                    {
                        continue;
                    }
                    _state.board.set_node_type(node.x, node.y, NodeType::Path);
                }
            }
        }
    }

    _state.board.draw(false);
    draw_text(
        format!("Current mode: {:?}", _state.place_mode).as_str(),
        vec2(-5.0, 5.0),
        RED,
        TextAlign::Center,
    );
}
