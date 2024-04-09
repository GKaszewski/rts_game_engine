#[allow(dead_code)]
use comfy::*;
use pathfinding::prelude::astar;

use super::PATHFINDING_Z_INDEX;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NodeType {
    Empty,
    Walkable,
    Unwalkable,
    Path,
    Start,
    End,
    Neighbor,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub node_type: Option<NodeType>,
}

impl Node {
    fn distance(&self, other: &Node) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let d = (dx * dx + dy * dy).sqrt();
        d
    }

    fn manhattan_distance(&self, other: &Node) -> f32 {
        let dx = (self.x - other.x).abs() as f32;
        let dy = (self.y - other.y).abs() as f32;
        dx + dy
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub node: Node,
    pub cost: u32,
}

impl PartialEq<(Node, u32)> for Successor {
    fn eq(&self, other: &(Node, u32)) -> bool {
        self.node == other.0 && self.cost == other.1
    }
}

pub struct PathfindingTerrain {
    pub width: i32,
    pub height: i32,
    pub nodes: Vec<Node>,
    pub offset: Option<Vec2>,
}

impl PathfindingTerrain {
    pub fn new(width: i32, height: i32, offset: Option<Vec2>) -> Self {
        let mut nodes = Vec::new();
        for y in 0..height {
            for x in 0..width {
                nodes.push(Node {
                    x,
                    y,
                    node_type: Some(NodeType::Walkable),
                });
            }
        }
        Self {
            width,
            height,
            nodes,
            offset,
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.nodes.clear();
        for y in 0..height {
            for x in 0..width {
                self.nodes.push(Node {
                    x,
                    y,
                    node_type: Some(NodeType::Empty),
                });
            }
        }
    }

    fn fill(&mut self, node_type: NodeType) {
        for node in &mut self.nodes {
            node.node_type = Some(node_type);
        }
    }

    // in other words, get neighbors
    pub fn get_successors(&self, node: &Node) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i32..=1i32 {
            for dy in -1i32..=1i32 {
                if (dx + dy).abs() != 1 {
                    continue;
                }

                let new_x = node.x + dx;
                let new_y = node.y + dy;

                if new_x < 0 || new_x >= self.width || new_y < 0 || new_y >= self.height {
                    continue;
                }

                match self.get_node_by_position(new_x, new_y) {
                    Some(n) => match n.node_type {
                        Some(NodeType::Unwalkable) => continue,
                        _ => {
                            successors.push(Successor { node: n, cost: 1 });
                        }
                    },
                    None => continue,
                }
            }
        }

        successors
    }

    pub fn set_node_type(&mut self, x: i32, y: i32, node_type: NodeType) {
        let node = &mut self.nodes[(y * self.width + x) as usize];
        node.node_type = Some(node_type);
    }

    pub fn get_node_by_position(&self, x: i32, y: i32) -> Option<Node> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(self.nodes[(y * self.width + x) as usize])
    }

    pub fn get_path(&self, start: Node, goal: Node) -> Option<Vec<Node>> {
        let result = astar(
            &start,
            |node| {
                self.get_successors(node)
                    .iter()
                    .map(|s| (s.node, s.cost))
                    .collect::<Vec<_>>()
            },
            |node| node.manhattan_distance(&goal) as u32,
            |node| *node == goal,
        );

        match result {
            Some((path, _cost)) => Some(path),
            None => None,
        }
    }

    pub fn clear_path(&mut self) {
        for node in &mut self.nodes {
            if node.node_type == Some(NodeType::Path) {
                node.node_type = Some(NodeType::Walkable);
            }
        }
    }

    pub fn set_pathfinding_based_on_tilemap(&mut self, level_map: &LdtkWorldMap) {
        let map = level_map;
        let map_json = &map.json;
        let level = &map_json.levels[1];
        for (_, layer) in level
            .layer_instances
            .as_ref()
            .unwrap()
            .iter()
            .rev()
            .enumerate()
        {
            let grid_size = layer.grid_size;
            let width = level.px_wid / grid_size;
            let height = level.px_hei / grid_size;

            self.resize(width as i32, height as i32);

            let tileset = layer
                .tileset_def_uid
                .and_then(|uid| map_json.defs.tilesets.iter().find(|t| t.uid == uid));

            if let Some(tileset) = tileset {
                tileset.enum_tags.iter().for_each(|tag| {
                    layer.grid_tiles.iter().for_each(|tile| {
                        let pos = tile.to_world(layer);
                        let x = pos.x as i32;
                        let y = pos.y as i32;
                        let node = self.get_node_by_position(x, y);
                        tag.tile_ids.iter().for_each(|tile_id| {
                            if tile.t == *tile_id {
                                match tag.enum_value_id.as_str() {
                                    "Start" => {
                                        if let Some(node) = node {
                                            self.set_node_type(node.x, node.y, NodeType::Start);
                                        }
                                    }
                                    "End" => {
                                        if let Some(node) = node {
                                            self.set_node_type(node.x, node.y, NodeType::End);
                                        }
                                    }
                                    "Unwalkable" => {
                                        if let Some(node) = node {
                                            self.set_node_type(
                                                node.x,
                                                node.y,
                                                NodeType::Unwalkable,
                                            );
                                        }
                                    }
                                    "Walkable" => {
                                        if let Some(node) = node {
                                            self.set_node_type(node.x, node.y, NodeType::Walkable);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        });
                    });
                });
            }
        }
    }

    pub fn draw(&self, display_coords: bool) {
        for node in &self.nodes {
            let color = match node.node_type {
                Some(NodeType::Empty) => BLACK,
                Some(NodeType::Walkable) => Color::new(0.0, 0.0, 0.0, 0.0),
                Some(NodeType::Unwalkable) => Color::new(255.0, 0.0, 0.0, 0.1),
                Some(NodeType::Path) => Color::new(0.0, 0.0, 255.0, 0.2),
                Some(NodeType::Start) => YELLOW,
                Some(NodeType::End) => YELLOW,
                Some(NodeType::Neighbor) => Color::new(255.0, 0.0, 255.0, 0.2),
                None => GRAY,
            };

            if let Some(offset) = self.offset {
                draw_rect(
                    vec2(node.x as f32, node.y as f32) + offset,
                    vec2(1.0, 1.0),
                    color,
                    PATHFINDING_Z_INDEX,
                );
            } else {
                draw_rect(
                    vec2(node.x as f32, node.y as f32),
                    vec2(1.0, 1.0),
                    color,
                    PATHFINDING_Z_INDEX,
                );
            }
            if display_coords {
                draw_text(
                    &format!("[{},{}]", node.x, node.y),
                    vec2(node.x as f32, node.y as f32),
                    WHITE,
                    TextAlign::Center,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathfinding() {
        let mut terrain = PathfindingTerrain::new(2, 2, None);
        terrain.set_node_type(0, 0, NodeType::Start);
        terrain.set_node_type(1, 1, NodeType::End);

        let start = terrain.get_node_by_position(0, 0).unwrap();
        let end = terrain.get_node_by_position(1, 1).unwrap();

        println!("Start: {:?}", start);
        println!("End: {:?}", end);

        let path = terrain.get_path(start, end);
        println!("Path: {:?}", path);

        assert!(path.is_some());
    }
}
