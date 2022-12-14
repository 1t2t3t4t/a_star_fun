use ecs::manager::EntityManager;
use macroquad::color::{BLACK, BLUE, RED, WHITE};
use macroquad::prelude::{draw_circle, draw_line, draw_rectangle, draw_text, get_frame_time, get_screen_data};
use macroquad::text::measure_text;
use macroquad::time::get_fps;
use macroquad::window::{screen_height, screen_width};
use crate::component::CameraMovement;
use crate::component::tag::Camera;
use crate::grid::GridState;
use crate::Grids;

pub fn fps() {
    let delta_time = get_frame_time();
    let fps = get_fps();
    let text = format!("FPS: {}, {}", fps, delta_time);
    let text_size = measure_text(&text, None, 15, 1f32);
    draw_text(&text, screen_width() - text_size.width, screen_height() - text_size.height, 15f32, BLACK);
}

pub fn draw_grids(manager: &mut EntityManager, grids: &mut Grids) {
    let Some((movement, _)) = manager.query_entities_components_one::<(CameraMovement, Camera)>() else {
        return
    };

    for grid_x in grids {
        for grid in grid_x {
            let size = grid.size;
            let (top_x, top_y) = grid.pos(movement.draw_offset);
            let (cen_x, cen_y) = grid.center(movement.draw_offset);

            draw_line(top_x, top_y, top_x + size, top_y, 3.0, BLUE);
            draw_line(top_x + size, top_y, top_x + size, top_y + size, 3.0, BLUE);
            draw_line(top_x + size, top_y + size, top_x, top_y + size, 3.0, BLUE);
            draw_line(top_x, top_y + size, top_x, top_y, 3.0, BLUE);

            draw_rectangle(
                top_x as f32,
                top_y as f32,
                grid.size as f32,
                grid.size as f32,
                grid.color(),
            );

            if let GridState::Path(step) = grid.state {
                draw_text(&format!("{}", step), cen_x, cen_y, 20f32, WHITE);
            } else {
                draw_circle(cen_x, cen_y, 5f32, RED);
            }

            draw_text(
                &format!("({}, {})", grid.x, grid.y),
                top_x,
                top_y,
                15f32,
                BLACK,
            )
        }
    }
}