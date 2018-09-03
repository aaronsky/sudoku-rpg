use input;
use sudoku::{self, Element, Generate, Grid, Point as KuPoint, Puzzle, Solve, Sudoku};

#[derive(Debug, Copy, Clone)]
pub struct Point(pub u8, pub u8);
impl Into<KuPoint> for Point {
    fn into(self) -> KuPoint {
        let Point(x, y) = self;
        let mut new_point = KuPoint::origin();
        new_point[0] = x;
        new_point[1] = y;
        new_point
    }
}

const ORDER: u8 = 3;

pub struct Gameboard {
    problem: Sudoku,
    pub current: Sudoku,
    pub solution: Sudoku,
    pub moves: usize,
    pub selected_cell: Option<Point>,
}

impl Gameboard {
    pub fn new(difficulty: sudoku::Difficulty) -> Self {
        let problem = Sudoku::generate(ORDER, difficulty);
        let current = problem.clone();
        let solution = problem.solution().unwrap();
        Self {
            problem,
            current,
            solution,
            moves: 0,
            selected_cell: None,
        }
    }

    pub fn insertion_is_correct(&self, point: Point, value: Element) -> bool {
        self.solution[point.into()] == Some(value)
    }

    pub fn insert(&mut self, point: Point, value: Element) {
        self.current = self.current.substitute(point.into(), Some(value));
        self.moves += 1;
    }

    pub fn remove(&mut self, point: Point) -> Option<Element> {
        self.moves += 1;
        let value = self.current[point.into()];
        self.current = self.current.substitute(point.into(), None);
        value
    }

    pub fn is_mutable(&self, point: Point) -> bool {
        self.problem[point.into()].is_none()
    }

    pub fn size(&self) -> [u8; sudoku::DIMENSIONS] {
        let order = self.solution.order() as u8;
        let dim = order.pow(sudoku::DIMENSIONS as u32);
        [dim; sudoku::DIMENSIONS]
    }

    pub fn points(&self) -> Vec<KuPoint> {
        self.current.points()
    }

    pub fn is_solved(&self) -> bool {
        self.current == self.solution
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

        let [nrows, ncols] = self.size();
        let [center_x, center_y] = [nrows / 2, ncols / 2];

        self.selected_cell = match self.selected_cell {
            Some(ref current) => {
                let [x, y] = [current.0, current.1];
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
                Some(Point(new_x, new_y))
            }
            None => Some(Point(center_x, center_y)),
        }
    }
}
