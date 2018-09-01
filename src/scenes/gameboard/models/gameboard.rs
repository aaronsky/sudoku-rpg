use input;

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

    pub fn move_selection(&mut self, axis: input::Axis, is_positive: bool) {
        let checked_add = |num1, num2| {
            if num1 == SIZE - 1 {
                0
            } else {
                num1 + num2
            }
        };
        let checked_sub = |num1, num2| {
            if num1 == 0 {
                SIZE - 1
            } else {
                num1 - num2
            }
        };

        self.selected_cell = match self.selected_cell {
            Some(current) => {
                let [x, y] = current;
                let new_x = if axis == input::Axis::Horz {
                    match is_positive {
                        true => checked_add(x, 1),
                        false => checked_sub(x, 1)
                    }
                } else {
                    x
                };
                let new_y = if axis == input::Axis::Vert {
                    match is_positive {
                        false => checked_add(y, 1),
                        true => checked_sub(y, 1),
                    }
                } else {
                    y
                };
                Some([new_x, new_y])
            },
            None => Some([0, 0]),
        }
    }
}
