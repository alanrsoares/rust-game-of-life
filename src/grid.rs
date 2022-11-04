use std::collections::HashMap;

const DEAD_CELL: char = '⬛';
const LIVE_CELL: char = '⬜';

#[derive(Debug)]
pub enum GridError {
    CellOutOfBoundsError,
    UnknownError,
}

#[derive(Debug, Clone)]
pub struct Grid {
    width: i32,
    height: i32,
    cells: HashMap<(i32, i32), Cell>,
}

impl Grid {
    fn new(width: i32, height: i32) -> Grid {
        let mut cells = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                cells.insert((x, y), Cell::new(x, y, false));
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn random(width: i32, height: i32) -> Grid {
        let mut cells = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                cells.insert((x, y), Cell::new(x, y, rand::random()));
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.cells.get(&(x, y))
    }

    pub fn cell_neighbors(&self, x: i32, y: i32) -> Option<Vec<Cell>> {
        let opt_cell = self.cell(x, y);

        if opt_cell.is_none() {
            return None;
        }

        let cell = opt_cell.unwrap();

        Some(cell.neighbours(self))
    }

    pub fn toggle_cell(&mut self, x: i32, y: i32) -> &Grid {
        let opt_cell = self.cell(x, y);

        if opt_cell.is_none() {
            panic!("Error: {:?}", GridError::CellOutOfBoundsError);
        }

        let mut new_cell = opt_cell.unwrap().clone();

        self.cells.insert((x, y), new_cell.toggle());

        self
    }

    pub fn next_state(&mut self) -> &Grid {
        let mut this = self.clone();

        self.cells.iter().for_each(|(key, cell)| {
            let live_neighbors = cell.live_neighbours_count(&this);

            this.cells.insert(*key, cell.next_state(live_neighbors));
        });

        self.cells = this.cells;

        self
    }

    pub fn render(&self) {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell(x, y).unwrap();
                output.push(if cell.is_alive { LIVE_CELL } else { DEAD_CELL });
            }
            output.push('\n');
        }

        // render output to terminal and clear screen
        println!("{}{}", termion::clear::All, output);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: i32,
    y: i32,
    is_alive: bool,
}

impl Cell {
    fn new(x: i32, y: i32, is_alive: bool) -> Cell {
        Cell {
            x: x,
            y: y,
            is_alive,
        }
    }

    fn neighbours(&self, grid: &Grid) -> Vec<Cell> {
        let coordinates = [
            (self.x - 1, self.y - 1),
            (self.x, self.y - 1),
            (self.x + 1, self.y - 1),
            (self.x - 1, self.y),
            (self.x + 1, self.y),
            (self.x - 1, self.y + 1),
            (self.x, self.y + 1),
            (self.x + 1, self.y + 1),
        ];

        coordinates
            .iter()
            .filter_map(|(x, y)| grid.cell(*x, *y))
            .map(|cell| *cell)
            .collect::<Vec<Cell>>()
    }

    pub fn live_neighbours_count(&self, grid: &Grid) -> usize {
        self.neighbours(grid)
            .iter()
            .filter(|cell| cell.is_alive)
            .collect::<Vec<&Cell>>()
            .len()
    }

    pub fn toggle(&mut self) -> Cell {
        self.is_alive = !self.is_alive;
        *self
    }

    /// # next_state
    /// Returns the next state of the cell given the number of live neighbours.
    ///
    /// ## Rules
    /// 1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    /// 2. Any live cell with two or three live neighbours lives on to the next generation.
    /// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    ///
    /// Example:
    /// ```
    /// use game_of_life::grid::Grid;
    ///
    /// let mut grid = Grid::new(3, 3);
    ///
    /// let cell = grid.cell(1, 1).unwrap();
    ///
    /// assert_eq!(cell.next_state(0).is_alive, false);
    ///
    /// ```
    pub fn next_state(&self, live_neighbour_count: usize) -> Cell {
        let mut cell = self.clone();

        match (self.is_alive, live_neighbour_count) {
            (true, 0 | 1) => cell.is_alive = false,
            (true, 2 | 3) => cell.is_alive = true,
            (true, _) => cell.is_alive = false,
            (false, 3) => cell.is_alive = true,
            (false, _) => cell.is_alive = false,
        }

        cell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_random_grid() {
        let columns = 10;
        let rows = 10;
        let grid = Grid::random(columns, rows);

        assert_eq!(grid.cells.keys().count(), (columns * rows) as usize);
    }
}
