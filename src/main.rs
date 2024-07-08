use macroquad::prelude::*;
use player::Player;
use renderer::{Renderer, FOV, TILE};
use map::procces_map;

pub mod map;
pub mod player;
pub mod renderer;

pub struct Config {
    resolution: f32,
    dt_angle: f32,
    projk: f32,
    scale: f32,
}

fn generate_cfg(resolution: f32) -> Config {
    let scale = screen_width() / resolution;
    let dist = resolution / (2.0 * (FOV / 2.0).tan());
    Config {
        resolution,
        dt_angle: FOV / resolution,
        projk: scale * dist * TILE,
        scale,
    }
}

#[macroquad::main("Engine")]
async fn main() {
    let cfg = generate_cfg(screen_width());
    set_pc_assets_folder("assets");

    let renderer = Renderer {
        texture: load_texture("1.png").await.unwrap(),
    };

    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0, 500.0, 0.0);
    let map = procces_map(vec![
        "#####################################",
        "#......###..........................#",
        "#.....#...#.........................#",
        "#.....#...#.........................#",
        "#......#.#................#.........#",
        "#.............#.....................#",
        "#.............#.....................#",
        "#.............###...................#",
        "#....#........#.....................#",
        "#....#........#.....................#",
        "#....#............................#.#",
        "#....#............................#.#",
        "#....#........#########........####.#",
        "#....#..............###...........#.#",
        "#...................................#",
        "#####################################",
    ]);

    loop {
        player.update();

        Renderer::background();
        renderer.render_main(map.clone(), player.x, player.y, player.angle, &cfg);
        Renderer::minimap(map.clone(), player.x, player.y, player.angle);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);
        next_frame().await
    }
}
