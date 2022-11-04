mod grid;

fn main() {
    let random_grid = grid::Grid::random(9, 9);

    let neighbors = random_grid.cell_neighbors(3, 3);

    println!("{:?}", neighbors);
}
