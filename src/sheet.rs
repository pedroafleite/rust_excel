use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Formula(String),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub value: CellValue,
    pub row: usize,
    pub col: usize,
}

#[derive(Default)]
pub struct Sheet {
    pub cells: HashMap<(usize, usize), Cell>,
}

impl Sheet {
    pub fn new() -> Self {
        Sheet {
            cells: HashMap::new(),
        }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: CellValue) {
        let cell = Cell { value, row, col };
        self.cells.insert((row, col), cell);
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(&(row, col))
    }
}