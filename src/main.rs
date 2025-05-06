use crate::game::{AlgebraicNotation, Game};
use crate::grid::{CellId, Grid};
use crate::input::{left_click_handler, on_selected};
use crate::pieces::{Piece, PieceType, Side};
use crate::textures::PieceTxts;
use macroquad::prelude::*;
mod game;
mod grid;
mod input;
mod path;
mod pieces;
mod textures;

const VIRTUAL_WIDTH: f32 = 2048.0;
const VIRTUAL_HEIGHT: f32 = 2048.0;

#[macroquad::main("Chess")]
async fn main() {
    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    set_pc_assets_folder("assets");
    let mut grid = Grid::new64(VIRTUAL_WIDTH / 8.0);
    let box_txts: Box<PieceTxts> = Box::new(PieceTxts::default().await);
    let piecetxts: &PieceTxts = Box::leak(box_txts);
    build_textures_atlas();

    let mut game = Game::new(&mut grid, piecetxts);
    let mut selected_cell: Option<CellId> = None;
    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
    render_target_cam.render_target = Some(render_target.clone());
    loop {
        // Get required scaling value
        let scale: f32 = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );
        let flip: bool = match game.turn {
            Side::White => false,
            Side::Black => true,
        };

        // Mouse position in the virtual screen
        let virtual_mouse_pos;
        if !flip {
            render_target_cam.rotation = 0.0;
            virtual_mouse_pos = Vec2 {
                x: (mouse_position().0 - (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.05) / scale,
                y: (mouse_position().1 - (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.05)
                    / scale,
            };
        } else {
            render_target_cam.rotation = 180.0;
            virtual_mouse_pos = Vec2 {
                x: 2048.0
                    - ((mouse_position().0 - (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.05)
                        / scale),
                y: 2048.0
                    - (mouse_position().1 - (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.05)
                        / scale,
            };
        }

        set_camera(&render_target_cam);

        left_click_handler(
            &mut grid,
            &mut selected_cell,
            &mut game,
            &render_target_cam,
            virtual_mouse_pos,
        );

        grid.draw(flip);

        if let Some(cell) = &selected_cell {
            on_selected(&mut grid, cell, &mut game);
        }
        set_default_camera();
        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.05,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.05,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                source: None,

                rotation: 0.0,
                flip_x: false,
                flip_y: true,
                pivot: None,
            },
        );
        next_frame().await
    }
}
