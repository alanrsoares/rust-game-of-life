use std::collections::HashMap;

#[derive(Debug)]
pub enum GridError {
    CellOutOfBoundsError,
    UnknownError,
}

#[derive(Debug, Clone)]
pub struct Grid {
    width: u32,
    height: u32,
    cells: HashMap<(u32, u32), Cell>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Grid {
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

    pub fn random(width: u32, height: u32) -> Grid {
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

    pub fn cell(&self, x: u32, y: u32) -> Option<&Cell> {
        self.cells.get(&(x, y))
    }

    pub fn cell_neighbors(&self, x: u32, y: u32) -> Option<Vec<Cell>> {
        let opt_cell = self.cell(x, y);

        if opt_cell.is_none() {
            return None;
        }

        let cell = opt_cell.unwrap();

        Some(cell.neighbours(self))
    }

    pub fn toggle_cell(&mut self, x: u32, y: u32) -> &Grid {
        let opt_cell = self.cell(x, y);

        if opt_cell.is_none() {
            panic!("Error: {:?}", GridError::CellOutOfBoundsError);
        }

        let mut new_cell = opt_cell.unwrap().clone();

        self.cells.insert((x, y), new_cell.toggle());

        self
    }

    pub fn next_state(&mut self) -> &Grid {
        let this1 = self.clone();

        let live_cells = this1
            .cells
            .iter()
            .filter(|(_, cell)| cell.is_alive)
            .map(|(_, cell)| cell.clone())
            .collect::<Vec<Cell>>();

        let mut insert = |(x, y), cell: &Cell| {
            self.cells.insert((x, y), cell.next_state(live_cells.len()));
        };

        let _ = this1.cells.keys().into_iter().map(|key| {
            let cell = this1.cells.get(key).unwrap();
            let neighbours = cell.neighbours(&this1);
            let live_neighbr_count = neighbours.into_iter().filter(|c| c.is_alive).count();
            let new_cell = cell.next_state(live_neighbr_count);
            insert(*key, &new_cell);
        });

        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: u32,
    y: u32,
    is_alive: bool,
}

impl Cell {
    fn new(x: u32, y: u32, is_alive: bool) -> Cell {
        Cell {
            x: x,
            y: y,
            is_alive,
        }
    }

    fn neighbours(&self, grid: &Grid) -> Vec<Cell> {
        let mut neighbours = Vec::new();

        for y in self.y - 1..self.y + 1 {
            for x in self.x - 1..self.x + 1 {
                if x == self.x && y == self.y {
                    continue;
                }
                if let Some(cell) = grid.cells.get(&(x, y)) {
                    neighbours.push(*cell);
                }
            }
        }

        neighbours
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
    /// assert_eq!(cell.next_state(1).is_alive, false);
    /// assert_eq!(cell.next_state(2).is_alive, true);
    /// assert_eq!(cell.next_state(3).is_alive, true);
    /// assert_eq!(cell.next_state(4).is_alive, false);
    /// assert_eq!(cell.next_state(5).is_alive, false);
    /// assert_eq!(cell.next_state(6).is_alive, false);
    /// assert_eq!(cell.next_state(7).is_alive, false);
    /// assert_eq!(cell.next_state(8).is_alive, false);
    ///
    /// ```
    pub fn next_state(&self, live_neighbour_count: usize) -> Cell {
        let mut new_cell = self.clone();

        match live_neighbour_count {
            2 => new_cell,
            3 => {
                new_cell.is_alive = true;
                new_cell
            }
            _ => {
                new_cell.is_alive = false;
                new_cell
            }
        }
    }
}
