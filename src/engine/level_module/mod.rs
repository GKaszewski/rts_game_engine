use comfy::*;

use super::TILEMAP_Z_INDEX;

pub const RTS_LEVEL_LDTK: &str = "assets/levels/rts.ldtk";
pub const RTS_GRID_SIZE: f32 = 64.0;

pub struct Map {
    pub map: LdtkWorldMap,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: LdtkWorldMap::new(
                parse_ldtk_map(include_str!("../../../assets/levels/rts.ldtk")).unwrap(),
                RTS_LEVEL_LDTK,
            ),
        }
    }

    pub fn initialize(&mut self, c: &mut EngineContext) {
        c.load_texture_from_bytes(
            "tileset",
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/sprites/rts_tileset.png"
            )),
        )
    }

    pub fn draw(&self) {
        let map_json = &self.map.json;
        let level = &map_json.levels[0];
        for (i, layer) in level
            .layer_instances
            .as_ref()
            .unwrap()
            .iter()
            .rev()
            .enumerate()
        {
            let grid_size = layer.grid_size as f32;

            let tileset = layer
                .tileset_def_uid
                .and_then(|uid| map_json.defs.tilesets.iter().find(|t| t.uid == uid));

            if let Some(_tileset) = tileset {
                let texture = texture_id("tileset");

                for tile in layer.grid_tiles.iter() {
                    let pos = tile.to_world(layer);

                    draw_sprite_ex(
                        texture,
                        pos,
                        WHITE,
                        TILEMAP_Z_INDEX + i as i32,
                        DrawTextureParams {
                            dest_size: Some(splat(grid_size / RTS_GRID_SIZE).as_world_size()),
                            source_rect: Some(IRect::new(
                                ivec2(tile.src[0] as i32, tile.src[1] as i32),
                                ivec2(grid_size as i32, grid_size as i32),
                            )),
                            flip_x: tile.f == 1 || tile.f == 3,
                            flip_y: tile.f == 2 || tile.f == 3,
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
