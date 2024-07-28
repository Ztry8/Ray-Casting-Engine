use crate::Config;
use macroquad::prelude::*;
use std::f32::consts::PI;

pub const FOV: f32 = PI / 3.0;
pub const TILE: f32 = 50.0;
const MINIMAP_SCALE: f32 = 5.0;
const TEXTURE_SIZE: f32 = 1024.0;
const TEXTURE_SCALE: f32 = TEXTURE_SIZE / TILE;

pub struct Renderer {
    pub texture: Texture2D,
}

impl Renderer {
    pub fn background() {
        clear_background(SKYBLUE);
        draw_rectangle(
            0.0,
            screen_height() / 2.0,
            screen_width(),
            screen_height() / 2.0,
            GRAY,
        );
    }

    pub fn minimap(body: Vec<(usize, usize)>, px: f32, py: f32, pa: f32) {
        draw_circle(
            px / MINIMAP_SCALE,
            py / MINIMAP_SCALE,
            10.0 / MINIMAP_SCALE,
            BLACK,
        );
        draw_line(
            px / MINIMAP_SCALE,
            py / MINIMAP_SCALE,
            px / MINIMAP_SCALE + 20.0 * pa.cos(),
            py / MINIMAP_SCALE + 20.0 * pa.sin(),
            1.0,
            BLACK,
        );

        for (x, y) in body {
            draw_rectangle_lines(
                x as f32 / MINIMAP_SCALE,
                y as f32 / MINIMAP_SCALE,
                TILE / MINIMAP_SCALE,
                TILE / MINIMAP_SCALE,
                2.0,
                BLACK,
            );
        }
    }

    fn mapping(a: usize, b: usize) -> (f32, f32) {
        (
            (a / TILE as usize) as f32 * TILE,
            ((b / TILE as usize) as f32 * TILE),
        )
    }

    pub fn render_main(&self, body: Vec<(usize, usize)>, px: f32, py: f32, pa: f32, cfg: &Config) {
        let (xm, ym) = Renderer::mapping(px as usize, py as usize);
        let mut cur_angle = pa - FOV / 2.0;
        for ray in 0..cfg.resolution as usize {
            let sin_a = cur_angle.sin();
            let cos_a = cur_angle.cos();

            /////
            let mut x = xm;
            let mut dx = -1.0;
            if cos_a >= 0.0 {
                x += TILE;
                dx = 1.0;
            }

            let mut depth_v = 0.0;
            let mut yv = 0.0;
            for _i in (0..screen_width() as usize).step_by(2) {
                depth_v = (x - px) / cos_a;
                yv = py + depth_v * sin_a;

                let (mapx, mapy) = Renderer::mapping((x + dx) as usize, yv as usize);
                if body.contains(&(mapx as usize, mapy as usize)) {
                    break;
                }

                x += dx * TILE;
            }

            /////
            let mut y = ym;
            let mut dy = -1.0;
            if sin_a >= 0.0 {
                y += TILE;
                dy = 1.0;
            }

            let mut xh = 0.0;
            let mut depth_h = 0.0;
            for _i in (0..screen_height() as usize).step_by(2) {
                depth_h = (y - py) / sin_a;
                xh = px + depth_h * cos_a;

                let (mapx, mapy) = Renderer::mapping(xh as usize, (y + dy) as usize);
                if body.contains(&(mapx as usize, mapy as usize)) {
                    break;
                }

                y += dy * TILE;
            }

            /////
            let mut depth = depth_h;
            let mut offset = xh;
            if depth_v < depth_h {
                depth = depth_v;
                offset = yv;
            }

            offset = (offset as usize % TILE as usize) as f32;

            depth *= (pa - cur_angle).cos().max(0.0001);
            let projh = (cfg.projk / depth).min(screen_height() * 2.0);
            let c = 1.0 / (depth * 0.005);

            draw_texture_ex(
                &self.texture,
                ray as f32 * cfg.scale,
                screen_height() / 2.0 - projh / 2.0,
                Color::new(c, c, c, 1.0),
                DrawTextureParams {
                    dest_size: Some(Vec2::new(cfg.scale, projh)),
                    source: Some(Rect::new(
                        offset * TEXTURE_SCALE,
                        0.0,
                        TEXTURE_SCALE,
                        TEXTURE_SIZE,
                    )),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );

            cur_angle += cfg.dt_angle;
        }
    }
}
