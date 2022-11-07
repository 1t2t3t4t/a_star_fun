use ecs::manager::EntityManager;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::get_frame_time;
use crate::component::CameraMovement;
use crate::component::tag::Camera;

const MOVEMENT_ACC: f32 = 1200f32;
const BRAKE: f32 = 500f32;

pub fn camera_movement_system(entity_manager: &mut EntityManager) {
    let components = entity_manager.query_entities_component_tag_mut::<CameraMovement, Camera>();
    for movement in components {
        let mut has_movement = false;

        if is_key_down(KeyCode::W) {
            has_movement = true;
            movement.velocity.1 += MOVEMENT_ACC * get_frame_time();
        } else if is_key_down(KeyCode::S) {
            has_movement = true;
            movement.velocity.1 -= MOVEMENT_ACC * get_frame_time();
        }

        if is_key_down(KeyCode::A) {
            has_movement = true;
            movement.velocity.0 += MOVEMENT_ACC * get_frame_time();
        } else if is_key_down(KeyCode::D) {
            has_movement = true;
            movement.velocity.0 -= MOVEMENT_ACC * get_frame_time();
        }

        if !has_movement {
            let reverse = (movement.velocity.0 * -1f32, movement.velocity.1 * -1f32);
            let r_size = (reverse.0.powi(2) + reverse.1.powi(2)).sqrt();
            if r_size != 0f32 {
                let reverse_norm = (reverse.0 / r_size, reverse.1 / r_size);
                let brake_dir = (
                    reverse_norm.0 * BRAKE * get_frame_time(),
                    reverse_norm.1 * BRAKE * get_frame_time(),
                );
                movement.velocity.0 += brake_dir.0.clamp(-reverse.0.abs(), reverse.0.abs());
                movement.velocity.1 += brake_dir.1.clamp(-reverse.1.abs(), reverse.1.abs());
            }
        }

        movement.draw_offset.0 += movement.velocity.0 * get_frame_time();
        movement.draw_offset.1 += movement.velocity.1 * get_frame_time();
    }
}