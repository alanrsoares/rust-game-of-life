use game_of_life::Grid;

fn main() {
    const GENERATIONS: usize = 60;
    const DELAY: u64 = 160;

    let mut new_grid = Grid::random(24, 24);

    for _ in 0..GENERATIONS {
        new_grid.next_state().render();

        // delay printing to console
        std::thread::sleep(std::time::Duration::from_millis(DELAY));
    }
}
