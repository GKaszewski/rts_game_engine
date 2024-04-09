use comfy::{hecs::With, serde_json::de, *};
use pathfinding::matrix::directions::S;

use crate::GameState;

use super::{selection_module::SelectedUnit, UNIT_Z_INDEX};

pub struct Unit;
pub struct UnitPath {
    pub path: Vec<Vec2>,
    pub current_node: usize,
}

pub struct MovePoint {
    pub point: Vec2,
    pub time: f32,
}

pub struct CollisionAvoidance {
    pub radius: f32,
}

pub fn initialize_units() {
    commands().spawn((
        Transform::position(vec2(10.0, 2.1)),
        Unit,
        CollisionAvoidance { radius: 0.5 },
    ));
    commands().spawn((
        Transform::position(vec2(11.2, 2.1)),
        Unit,
        CollisionAvoidance { radius: 0.5 },
    ));
    commands().spawn((
        Transform::position(vec2(11.5, 2.1)),
        Unit,
        CollisionAvoidance { radius: 0.5 },
    ));
}

pub fn get_path_for_selected_units_system(state: &mut GameState) {
    if is_mouse_button_pressed(MouseButton::Right) {
        let target = mouse_world();

        for (entity, transform) in world().query::<With<&Transform, &SelectedUnit>>().iter() {
            commands().remove_one::<&UnitPath>(entity);

            let start_node = state
                .board
                .get_node_by_position(transform.position.x as i32, transform.position.y as i32);
            let goal_node = state
                .board
                .get_node_by_position(target.x as i32, target.y as i32);
            if let Some((start_node, goal_node)) = start_node.zip(goal_node) {
                let path = state.board.get_path(start_node, goal_node);
                if let Some(path) = path {
                    let points = path
                        .iter()
                        .map(|node| vec2(node.x as f32, node.y as f32))
                        .collect();
                    commands().insert_one(
                        entity,
                        UnitPath {
                            path: points,
                            current_node: 0,
                        },
                    );
                    commands().spawn((
                        MovePoint {
                            point: target,
                            time: 0.0,
                        },
                        Transform::position(target),
                    ));
                }
            }
        }
    }
}

pub fn move_units_along_path_system() {
    for (entity, (transform, path)) in world().query::<(&mut Transform, &mut UnitPath)>().iter() {
        if path.current_node < path.path.len() {
            let target = path.path[path.current_node];
            let direction = target - transform.position;
            let distance = direction.length();
            let speed = 10.0;
            let velocity = direction.normalize() * speed * delta();
            if distance < 0.1 {
                path.current_node += 1;
            } else {
                transform.position += velocity;
            }
        } else {
            commands().remove_one::<&UnitPath>(entity);
        }
    }
}

pub fn update_move_point_timer_system() {
    for (_, (move_point, _)) in world().query::<(&mut MovePoint, &Transform)>().iter() {
        move_point.time += delta();
    }
}

pub fn cleanup_move_points_system() {
    for (entity, (move_point, _)) in world().query::<(&MovePoint, &Transform)>().iter() {
        if move_point.time > 1.0 {
            commands().despawn(entity);
        }
    }
}

pub fn collision_avoidance_system() {
    let mut unit_positions = Vec::new();

    for (entity, (transform, _, avoidance)) in world()
        .query::<(&Transform, &Unit, &CollisionAvoidance)>()
        .iter()
    {
        unit_positions.push((entity, transform.position, avoidance.radius));
    }

    for (entity, position, radius) in &unit_positions {
        let mut avoidance_vector = Vec2::ZERO;

        for (other_entity, other_position, other_radius) in &unit_positions {
            if entity == other_entity {
                continue;
            }

            let distance = position.distance(*other_position);
            if distance < *radius + *other_radius {
                let direction_to_other = (*other_position - *position).normalize();
                avoidance_vector -=
                    direction_to_other * (radius + other_radius - distance) / distance;
            }
        }

        if avoidance_vector != Vec2::ZERO {
            if let Ok(mut transform_q) = world().query_one::<(&mut Transform, &Unit)>(*entity) {
                if let Some((transform, _)) = transform_q.get() {
                    transform.position += avoidance_vector * delta();
                }
            }
        }
    }
}

pub fn spawn_unit_at_mouse_position_system() {
    let mouse_pos = mouse_world();
    if is_key_pressed(KeyCode::Space) {
        commands().spawn((
            Transform::position(mouse_pos),
            Unit,
            CollisionAvoidance { radius: 0.5 },
        ));
    }
}

pub fn draw_units() {
    for (_, (transform, _)) in world().query::<(&Transform, &Unit)>().iter() {
        draw_circle(transform.position, 0.5, RED, UNIT_Z_INDEX);
    }
}

pub fn draw_move_points() {
    for (_, (transform, _)) in world().query::<(&Transform, &MovePoint)>().iter() {
        draw_circle(transform.position, 0.1, GREEN, UNIT_Z_INDEX);
    }
}
