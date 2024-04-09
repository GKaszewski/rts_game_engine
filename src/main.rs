use comfy::*;
mod engine;

use engine::game_module::*;
use engine::pathfinding_module::*;
use engine::selection_module::draw_selection_box;
use engine::selection_module::draw_selection_on_units;
use engine::selection_module::get_units_in_selection_system;
use engine::selection_module::initialize_selection_module;
use engine::selection_module::selection_box_system;
use engine::unit_module::draw_units;
use engine::unit_module::initialize_units;

simple_game!("RTS Game Engine", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(600 * 16 / 9, 600),
        ..config
    }
}

fn setup(state: &mut GameState, c: &mut EngineContext) {
    state.level.initialize(c);
    state.set_board_based_on_tilemap();
    initialize_units();
    initialize_selection_module(c);
}

fn place(state: &mut GameState, x: i32, y: i32) {
    match state.place_mode {
        PlaceMode::Start => {
            let node = Node {
                x,
                y,
                node_type: Some(NodeType::Start),
            };
            for y in 0..state.board.height {
                for x in 0..state.board.width {
                    let node = state.board.get_node_by_position(x, y);
                    if let Some(node) = node {
                        if node.node_type == Some(NodeType::Start) {
                            state.board.set_node_type(x, y, NodeType::Walkable);
                        }
                    }
                }
            }

            state.set_start(node);
            state.board.set_node_type(x, y, NodeType::Start);
        }
        PlaceMode::End => {
            for y in 0..state.board.height {
                for x in 0..state.board.width {
                    let node = state.board.get_node_by_position(x, y);
                    if let Some(node) = node {
                        if node.node_type == Some(NodeType::End) {
                            state.board.set_node_type(x, y, NodeType::Walkable);
                        }
                    }
                }
            }

            state.set_end(Node {
                x,
                y,
                node_type: Some(NodeType::End),
            });
            state.board.set_node_type(x, y, NodeType::End);
        }
        PlaceMode::Wall => {
            state.board.set_node_type(x, y, NodeType::Unwalkable);
        }
        PlaceMode::Walkable => {
            state.board.set_node_type(x, y, NodeType::Walkable);
        }
    }
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Num1) {
        state.place_mode = PlaceMode::Start;
    }
    if is_key_pressed(KeyCode::Num2) {
        state.place_mode = PlaceMode::End;
    }
    if is_key_pressed(KeyCode::Num3) {
        state.place_mode = PlaceMode::Wall;
    }
    if is_key_pressed(KeyCode::Num4) {
        state.place_mode = PlaceMode::Walkable;
    }

    if is_key_pressed(KeyCode::P) {
        state.draw_pathfinding = !state.draw_pathfinding;
    }

    selection_box_system();
    get_units_in_selection_system();

    // if is_mouse_button_down(MouseButton::Left) {
    //     let mouse_pos = mouse_world();
    //     // let node = match state.board.offset {
    //     //     Some(offset) => state.board.get_node_by_position(
    //     //         (mouse_pos.x - offset.x) as i32,
    //     //         (mouse_pos.y - offset.y) as i32,
    //     //     ),
    //     //     None => state
    //     //         .board
    //     //         .get_node_by_position(mouse_pos.x as i32, mouse_pos.y as i32),
    //     // };
    //     let node = state
    //         .board
    //         .get_node_by_position(mouse_pos.x as i32, mouse_pos.y as i32);
    //     println!("Clicked on node: {:?}", node);
    //     if let Some(node) = node {
    //         place(state, node.x, node.y);
    //     }
    // }

    state.rts_camera.update();

    if is_key_pressed(KeyCode::Space) {}

    state.board.clear_path();

    if let Some(start) = state.start {
        if let Some(end) = state.end {
            let path = state.board.get_path(start, end);
            if let Some(path) = path {
                for node in path {
                    if node.node_type == Some(NodeType::Start)
                        || node.node_type == Some(NodeType::End)
                    {
                        continue;
                    }
                    state.board.set_node_type(node.x, node.y, NodeType::Path);
                }
            }
        }
    }

    if state.draw_pathfinding {
        state.board.draw(false);
    }

    draw_text(
        format!("Current mode: {:?}", state.place_mode).as_str(),
        vec2(-6.0, 6.0),
        RED,
        TextAlign::Center,
    );

    state.level.draw();
    draw_units();
    draw_selection_box();
    draw_selection_on_units();
}
