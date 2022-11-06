mod grid;
mod path_finder;

use crate::grid::{Grid, GridState};
use macroquad::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

const GRID_SIZE_X: usize = 20;
const GRID_SIZE_Y: usize = 20;

const GRID_SIZE: f32 = 50f32;

const MOVEMENT_ACC: f32 = 1200f32;
const BRAKE: f32 = 500f32;

pub type Grids = Vec<Vec<Grid>>;

#[derive(Default, Debug)]
struct Context {
    start_pos: Option<(i32, i32)>,
    end_pos: Option<(i32, i32)>,
    block: HashSet<(i32, i32)>,
    draw_offset: (f32, f32),
    velocity: (f32, f32),
}

fn get_grid_pos_from_point(x: f32, y: f32, offset: (f32, f32)) -> Option<(usize, usize)> {
    let (x, y) = (x - offset.0, y - offset.1);
    let possible_pos = ((x / GRID_SIZE) as usize, (y / GRID_SIZE) as usize);
    if possible_pos.0 < GRID_SIZE_X && possible_pos.1 < GRID_SIZE_Y {
        Some(possible_pos)
    } else {
        None
    }
}

fn clear_all(ctx: &mut Context, grids: &mut Grids) {
    ctx.start_pos = None;
    ctx.end_pos = None;
    clean_path(grids);
}

fn clean_path(grids: &mut Grids) {
    for grid_x in grids {
        for grid in grid_x {
            if grid.state != GridState::Blocked {
                grid.state = GridState::Idle;
            }
        }
    }
}

fn elapsed<T>(f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let res = f();
    let elapsed = Instant::now().duration_since(start);
    println!("Process took {} ms.", elapsed.as_millis());
    res
}

fn force_find_path(ctx: &mut Context, grids: &mut Grids) {
    clean_path(grids);
    let start_pos = ctx.start_pos.unwrap();
    let end_pod = ctx.end_pos.unwrap();
    let block = ctx.block.clone();
    let path =
        elapsed(move || path_finder::a_star((GRID_SIZE_X, GRID_SIZE_Y), start_pos, end_pod, block));
    for (i, node) in path.iter().enumerate() {
        let grid = &mut grids[node.1 as usize][node.0 as usize];
        grid.state = GridState::Path(i);
    }
}

async fn game_loop(ctx: &mut Context, grids: &mut Grids) {
    let mouse_pos = mouse_position();
    let grid_pos = get_grid_pos_from_point(mouse_pos.0, mouse_pos.1, ctx.draw_offset);
    if is_mouse_button_down(MouseButton::Right) {
        if let Some((x, y)) = grid_pos {
            let grid = &mut grids[y][x];
            grid.state = GridState::Blocked;
            ctx.block.insert((x as i32, y as i32));
            if ctx.start_pos.is_some() && ctx.end_pos.is_some() {
                force_find_path(ctx, grids);
            }
        }
    } else if is_mouse_button_pressed(MouseButton::Left) {
        let Some((x, y)) = grid_pos else {
            return
        };
        let grid = &mut grids[y][x];
        if ctx.start_pos.is_none() {
            ctx.start_pos = Some((x as i32, y as i32));
            grid.state = GridState::Focus;
        } else if ctx.end_pos.is_none() {
            ctx.end_pos = Some((x as i32, y as i32));
            grid.state = GridState::Focus;
            force_find_path(ctx, grids);
        }
    }

    if is_key_pressed(KeyCode::Space) {
        clear_all(ctx, grids);
    }

    let mut has_movement = false;

    if is_key_down(KeyCode::W) {
        has_movement = true;
        ctx.velocity.1 += MOVEMENT_ACC * get_frame_time();
    } else if is_key_down(KeyCode::S) {
        has_movement = true;
        ctx.velocity.1 -= MOVEMENT_ACC * get_frame_time();
    }

    if is_key_down(KeyCode::A) {
        has_movement = true;
        ctx.velocity.0 += MOVEMENT_ACC * get_frame_time();
    } else if is_key_down(KeyCode::D) {
        has_movement = true;
        ctx.velocity.0 -= MOVEMENT_ACC * get_frame_time();
    }

    if !has_movement {
        let reverse = (ctx.velocity.0 * -1f32, ctx.velocity.1 * -1f32);
        let r_size = (reverse.0.powi(2) + reverse.1.powi(2)).sqrt();
        if r_size != 0f32 {
            let reverse_norm = (reverse.0 / r_size, reverse.1 / r_size);
            let brake_dir = (
                reverse_norm.0 * BRAKE * get_frame_time(),
                reverse_norm.1 * BRAKE * get_frame_time(),
            );
            ctx.velocity.0 += brake_dir.0.clamp(-reverse.0.abs(), reverse.0.abs());
            ctx.velocity.1 += brake_dir.1.clamp(-reverse.1.abs(), reverse.1.abs());
        }
    }

    ctx.draw_offset.0 += ctx.velocity.0 * get_frame_time();
    ctx.draw_offset.1 += ctx.velocity.1 * get_frame_time();
}

async fn draw_loop(ctx: &mut Context, grids: &mut Grids) {
    clear_background(WHITE);

    for grid_x in grids {
        for grid in grid_x {
            let size = grid.size;
            let (top_x, top_y) = grid.pos(ctx.draw_offset);
            let (cen_x, cen_y) = grid.center(ctx.draw_offset);

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

    next_frame().await
}

#[macroquad::main("AStarFun")]
async fn main() {
    let mut context = Context::default();
    let mut grids: Grids = grid::build_grid::<GRID_SIZE_X, GRID_SIZE_Y>(GRID_SIZE);
    loop {
        game_loop(&mut context, &mut grids).await;
        draw_loop(&mut context, &mut grids).await;
    }
}
