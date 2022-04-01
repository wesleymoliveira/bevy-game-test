use bevy::prelude::Component;

#[derive(Copy, Clone)]
pub struct Cell {
    pub index: u32,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { index: 0 }
    }
}

#[derive(Component)]
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            size,
            cells: vec![Cell::default(); size * size],
        }
    }
}
