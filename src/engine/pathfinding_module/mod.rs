use comfy::*;
use pathfinding::prelude::astar;

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
    pub offset: Vec2,
}

impl PathfindingTerrain {
    pub fn new(width: i32, height: i32, offset: Vec2) -> Self {
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

    pub fn draw(&self, display_coords: bool) {
        for node in &self.nodes {
            let color = match node.node_type {
                Some(NodeType::Empty) => BLACK,
                Some(NodeType::Walkable) => GREEN,
                Some(NodeType::Unwalkable) => RED,
                Some(NodeType::Path) => BLUE,
                Some(NodeType::Start) => YELLOW,
                Some(NodeType::End) => YELLOW,
                Some(NodeType::Neighbor) => ORANGE,
                None => unreachable!(),
            };
            draw_rect(
                vec2(node.x as f32, node.y as f32) + self.offset,
                vec2(1.0, 1.0),
                color,
                1,
            );
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
        let mut terrain = PathfindingTerrain::new(2, 2, vec2(0.0, 0.0));
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
