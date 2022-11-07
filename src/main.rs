mod grid;
mod path_finder;
mod component;
mod system;

use crate::grid::Grid;
use macroquad::prelude::*;
use std::time::Instant;
use ecs::manager::EntityManager;
use crate::component::{CameraMovement, GridBlock, GridPath};
use crate::component::tag::Camera;

const GRID_SIZE_X: usize = 20;
const GRID_SIZE_Y: usize = 20;

const GRID_SIZE: f32 = 50f32;

pub type Grids = Vec<Vec<Grid>>;

#[derive(Default)]
struct Context {
    entity_manager: EntityManager
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

fn elapsed<T>(f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let res = f();
    let elapsed = Instant::now().duration_since(start);
    println!("Process took {} ms.", elapsed.as_millis());
    res
}

async fn game_loop(ctx: &mut Context, grids: &mut Grids) {
    ctx.entity_manager.update();
    system::movement::camera_movement_system(&mut ctx.entity_manager);

    system::grid::grid_path_system(&mut ctx.entity_manager, grids);
    system::grid::grid_block_system(&mut ctx.entity_manager, grids);
    system::grid::clear_all_system(&mut ctx.entity_manager, grids);
    system::grid::find_path_system(&mut ctx.entity_manager, grids);
}

async fn draw_loop(ctx: &mut Context, grids: &mut Grids) {
    clear_background(WHITE);

    system::render::draw_grids(&mut ctx.entity_manager, grids);

    next_frame().await
}

#[macroquad::main("AStarFun")]
async fn main() {
    let mut context = Context::default();
    context.entity_manager.add_event_system();
    context.entity_manager
        .add()
        .add_component(GridPath::default())
        .add_component(GridBlock::default());
    context.entity_manager
        .add()
        .add_component(CameraMovement::default())
        .add_component(Camera);

    let mut grids: Grids = grid::build_grid::<GRID_SIZE_X, GRID_SIZE_Y>(GRID_SIZE);
    loop {
        game_loop(&mut context, &mut grids).await;
        draw_loop(&mut context, &mut grids).await;
    }
}
