use comfy::{hecs::With, *};

use super::{unit_module::Unit, UNIT_Z_INDEX};

pub struct SelectedUnit {}

pub struct SelectionBox {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
    pub height: f32,
}

pub fn initialize_selection_module(c: &mut EngineContext) {
    c.load_texture_from_bytes(
        "selection",
        include_bytes!("../../../assets/sprites/selection.png"),
    );

    commands().spawn((
        SelectionBox {
            start: vec2(0.0, 0.0),
            end: vec2(0.0, 0.0),
            width: 0.0,
            height: 0.0,
        },
        Transform::position(vec2(0.0, 0.0)),
    ));
}

pub fn selection_box_system() {
    match world().query::<&mut SelectionBox>().iter().next() {
        Some((_, selection_box)) => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let start = mouse_world();
                selection_box.start = start;
            }
            if is_mouse_button_down(MouseButton::Left) {
                let end = mouse_world();
                selection_box.end = end;
                selection_box.width = end.x - selection_box.start.x;
                selection_box.height = end.y - selection_box.start.y;
            }
            if is_mouse_button_released(MouseButton::Left) {
                selection_box.width = 0.0;
                selection_box.height = 0.0;
                selection_box.start = vec2(0.0, 0.0);
                selection_box.end = vec2(0.0, 0.0);
            }
        }
        None => return,
    }
}

pub fn get_units_in_selection_system() {
    let binding = world();
    let mut binding = binding.query::<&SelectionBox>();
    let selection_box_query = binding.iter().next();
    if let Some(selection_box_query) = selection_box_query {
        let (_, selection_box) = selection_box_query;

        for (entity, (transform, _)) in &mut world().query::<(&Transform, &Unit)>().iter() {
            let min_x = selection_box.start.x.min(selection_box.end.x);
            let max_x = selection_box.start.x.max(selection_box.end.x);

            let min_y = selection_box.start.y.min(selection_box.end.y);
            let max_y = selection_box.start.y.max(selection_box.end.y);

            if transform.position.x > min_x
                && transform.position.x < max_x
                && transform.position.y > min_y
                && transform.position.y < max_y
            {
                commands().insert_one(entity, SelectedUnit {});
            }
        }
    }
}

pub fn deselect_units_system() {
    if is_mouse_button_pressed(MouseButton::Left) {
        for (entity, _) in world().query::<&SelectedUnit>().iter() {
            commands().remove_one::<SelectedUnit>(entity);
        }
    }
}

pub fn draw_selection_box() {
    for (_, selection_box) in world().query::<&SelectionBox>().iter() {
        let center = Vec2::new(selection_box.width / 2.0, selection_box.height / 2.0);
        let size = Vec2::new(selection_box.width, selection_box.height);

        draw_rect_outline(selection_box.start + center, size, 0.2, WHITE, 100);
    }
}

pub fn draw_selection_on_units() {
    for (_, transform) in world().query::<With<&Transform, &SelectedUnit>>().iter() {
        draw_sprite_ex(
            texture_id("selection"),
            transform.position,
            WHITE,
            UNIT_Z_INDEX + 1,
            DrawTextureParams {
                dest_size: Some(splat(1.0).as_world_size()),
                ..Default::default()
            },
        );
    }
}
