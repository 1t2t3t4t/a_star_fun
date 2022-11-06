use macroquad::color::{Color, GRAY, GREEN, RED, WHITE};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub enum GridState {
    Idle,
    Blocked,
    Focus,
    Path(usize),
}

impl Default for GridState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Grid {
    pub x: usize,
    pub y: usize,
    pub size: f32,
    pub state: GridState,
}

impl Grid {
    pub fn new(x: usize, y: usize, size: f32) -> Self {
        Self {
            x,
            y,
            size,
            ..Default::default()
        }
    }

    pub fn pos(&self, offset: (f32, f32)) -> (f32, f32) {
        let offset_x = self.size * self.x as f32;
        let offset_y = self.size * self.y as f32;
        (offset_x + offset.0, offset_y + offset.1)
    }

    pub fn center(&self, offset: (f32, f32)) -> (f32, f32) {
        let (x, y) = self.pos(offset);
        (x + self.size / 2f32, y + self.size / 2f32)
    }

    pub fn is_pos_in_grid(&self, pos: (f32, f32), offset: (f32, f32)) -> bool {
        (pos.0 >= self.pos(offset).0 && pos.0 <= self.pos(offset).0 + self.size)
            && (pos.1 >= self.pos(offset).1 && pos.1 <= self.pos(offset).1 + self.size)
    }

    pub fn color(&self) -> Color {
        match self.state {
            GridState::Idle => GREEN,
            GridState::Blocked => RED,
            GridState::Focus => WHITE,
            GridState::Path(_) => GRAY,
        }
    }
}

pub fn build_grid<const X: usize, const Y: usize>(size: f32) -> Vec<Vec<Grid>> {
    let mut grids: Vec<Vec<Grid>> = Vec::with_capacity(Y);
    for y in 0..Y {
        let mut grid_x = Vec::with_capacity(X);
        for x in 0..X {
            let grid = Grid::new(x, y, size);
            grid_x.push(grid);
        }
        grids.push(grid_x);
    }
    grids
}
