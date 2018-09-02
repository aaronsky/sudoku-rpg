use ggez::nalgebra::{zero, MatrixN as Matrix, U9};
use input;
use std::collections::HashSet;

lazy_static! {
    static ref FIXED_SET: HashSet<u8> = (1..=9).collect();
}

pub struct Gameboard {
    cells: Matrix<u8, U9>,
    pub selected_cell: Option<(usize, usize)>,
    pub solved: bool,
}

impl Gameboard {
    pub fn new() -> Self {
        Gameboard {
            cells: zero(),
            selected_cell: None,
            solved: false,
        }
    }

    pub fn get(&self, ind: (usize, usize)) -> Option<u8> {
        match self.cells[ind] {
            cell @ 1..=9 => Some(cell),
            _ => None,
        }
    }

    pub fn set(&mut self, ind: (usize, usize), val: u8) {
        self.cells[ind] = val;
    }

    pub fn clear(&mut self, ind: (usize, usize)) {
        self.cells[ind] = 0;
    }

    pub fn move_selected_cell(&mut self, axis: input::Axis, is_positive: bool) {
        let checked_add = |num1, num2, max| {
            if num1 == max - 1 {
                0
            } else {
                num1 + num2
            }
        };
        let checked_sub = |num1, num2, max| {
            if num1 == 0 {
                max - 1
            } else {
                num1 - num2
            }
        };

        self.selected_cell = match self.selected_cell {
            Some(current) => {
                let (x, y) = current;
                let (nrows, ncols) = self.cells.shape();
                let new_x = if axis == input::Axis::Horz {
                    if is_positive {
                        checked_add(x, 1, nrows)
                    } else {
                        checked_sub(x, 1, nrows)
                    }
                } else {
                    x
                };
                let new_y = if axis == input::Axis::Vert {
                    if is_positive {
                        checked_sub(y, 1, ncols)
                    } else {
                        checked_add(y, 1, ncols)
                    }
                } else {
                    y
                };
                Some((new_x, new_y))
            }
            None => Some((0, 0)),
        }
    }

    pub fn check_solution(&self) -> bool {
        // check each row
        // check each column
        // check each grid
        false
    }

    fn check_row(&self, ind: usize) -> bool {
        // let row = self.cells.row(ind);
        // let blanks = row.iter().filter(|v| **v == 0).count();
        // if blanks > 0 {
        //     false
        // } else {
        //     let row_set: HashSet<_> = row.iter().filter(|v| **v != 0).collect();
        //     row_set.len() == FIXED_SET.len()
        // }
        false
    }

    fn check_col(&self, ind: usize) -> bool {
        // let column = self.cells.fold
        false
    }

    fn check_grid(&self, row: usize, col: usize) -> bool {
        false
    }
}
