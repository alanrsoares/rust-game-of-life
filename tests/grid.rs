use life::Grid;

#[test]
fn it_should_create_a_random_grid() {
    let columns = 10;
    let rows = 10;
    let grid = Grid::random(columns, rows);

    assert_eq!(grid.cells.keys().count(), (columns * rows) as usize);
}
