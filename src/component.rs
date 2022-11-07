use std::collections::HashSet;

pub mod tag {
    pub struct Camera;
}

#[derive(Default, Debug)]
pub struct CameraMovement {
    pub draw_offset: (f32, f32),
    pub velocity: (f32, f32),
}

#[derive(Default, Debug)]
pub struct GridBlock {
    pub block: HashSet<(i32, i32)>,
}

#[derive(Default, Debug)]
pub struct GridPath {
    pub start_pos: Option<(i32, i32)>,
    pub end_pos: Option<(i32, i32)>,
}