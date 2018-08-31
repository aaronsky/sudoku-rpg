const SIZE: usize = 9;

pub struct Gameboard {
    cells: [[u8; SIZE]; SIZE],
    pub selected_cell: Option<[usize; 2]>,
    pub solved: bool,
}

impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
            selected_cell: None,
            solved: false,
        }
    }

    pub fn get(&self, ind: [usize; 2]) -> Option<u8> {
        if let Some(cell) = self.cells.get(ind[1])?.get(ind[0]) {
            match cell {
                1..=9 => Some(*cell),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }
}
