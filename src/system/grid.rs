use ecs::event::{EventReceiver, EventSender, EventSystem};
use ecs::manager::EntityManager;
use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed, mouse_position, MouseButton};
use crate::{elapsed, get_grid_pos_from_point, GRID_SIZE_X, GRID_SIZE_Y, Grids, path_finder};
use crate::component::{CameraMovement, GridBlock, GridPath};
use crate::grid::GridState;

struct ClearPathEvent;
struct SubmitPath {
    start_pos: (i32, i32),
    end_pos: (i32, i32),
}

pub fn grid_block_system(manager: &mut EntityManager, grids: &mut Grids) {
    let movements = manager.query_entities_component::<CameraMovement>();
    let Some(camera_movement) = movements.first() else {
        return
    };
    let mouse_pos = mouse_position();
    let grid_pos = get_grid_pos_from_point(mouse_pos.0, mouse_pos.1, camera_movement.draw_offset);

    if is_mouse_button_down(MouseButton::Right) {
        if let Some((x, y)) = grid_pos {
            let grid = &mut grids[y][x];
            grid.state = GridState::Blocked;

            for block_component in manager.query_entities_component_mut::<GridBlock>() {
                block_component.1.block.insert((x as i32, y as i32));
            }

            let paths = manager.query_entities_component::<GridPath>();
            let Some(grid_path) = paths.first() else {
                return
            };

            if grid_path.start_pos.is_some() && grid_path.end_pos.is_some() {
                let submit = SubmitPath {
                    start_pos: grid_path.start_pos.unwrap(),
                    end_pos: grid_path.end_pos.unwrap()
                };
                let mut event = manager.query_entities_component_mut::<EventSystem>();
                let Some((_, event_system)) = event.first_mut() else {
                    return
                };
                event_system.send(submit);
            }
        }
    }
}

pub fn grid_path_system(manager: &mut EntityManager, grids: &mut Grids) {
    let movements = manager.query_entities_component::<CameraMovement>();
    let Some(camera_movement) = movements.first() else {
        return
    };
    let mouse_pos = mouse_position();
    let grid_pos = get_grid_pos_from_point(mouse_pos.0, mouse_pos.1, camera_movement.draw_offset);

    if is_mouse_button_pressed(MouseButton::Left) {
        let Some((x, y)) = grid_pos else {
            return
        };
        let mut paths = manager.query_entities_component_mut::<GridPath>();
        let Some((_, grid_path)) = paths.first_mut() else {
            return
        };
        if grid_path.start_pos.is_some() && grid_path.end_pos.is_some() {
            grid_path.start_pos = None;
            grid_path.end_pos = None;
        }
        let grid = &mut grids[y][x];
        if grid_path.start_pos.is_none() {
            grid_path.start_pos = Some((x as i32, y as i32));
            grid.state = GridState::Focus;
        } else if grid_path.end_pos.is_none() {
            grid_path.end_pos = Some((x as i32, y as i32));
            grid.state = GridState::Focus;
            let submit_path = SubmitPath {
                start_pos: grid_path.start_pos.unwrap(),
                end_pos: grid_path.end_pos.unwrap()
            };

            let mut event = manager.query_entities_component_mut::<EventSystem>();
            let Some((_, event_system)) = event.first_mut() else {
                return
            };
            event_system.send(submit_path);
        }
    }
}

pub fn clear_all_system(manager: &mut EntityManager, grids: &mut Grids) {
    let mut event = manager.query_entities_component_mut::<EventSystem>();
    let Some((_, event)) = event.first_mut() else {
        return
    };
    let events: Vec<ClearPathEvent> = event.read();
    if !events.is_empty() {
        let mut path = manager.query_entities_component_mut::<GridPath>();
        if let Some((_, path)) = path.first_mut() {
            path.start_pos = None;
            path.end_pos = None;
        }
        clean_path(grids);
    }
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

pub fn find_path_system(manager: &mut EntityManager, grids: &mut Grids) {
    let mut event = manager.query_entities_component_mut::<EventSystem>();
    let Some((_, event)) = event.first_mut() else {
        return
    };
    let events: Vec<SubmitPath> = event.read();
    let Some(submit_path) = events.first() else {
        return
    };
    let block = manager.query_entities_component::<GridBlock>();
    let Some(block) = block.first() else {
        return
    };
    clean_path(grids);
    let start_pos = submit_path.start_pos;
    let end_pod = submit_path.end_pos;
    let block = block.block.clone();
    let path =
        elapsed(move || path_finder::a_star((GRID_SIZE_X, GRID_SIZE_Y), start_pos, end_pod, block));
    for (i, node) in path.iter().enumerate() {
        let grid = &mut grids[node.1 as usize][node.0 as usize];
        grid.state = GridState::Path(i);
    }
}