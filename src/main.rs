use game_of_life::Grid;

fn main() {
    const MAX_GENERATIONS: usize = 100;
    const FRAME_DELAY: u64 = (1000 / 60) * 8;

    let mut current_generation = 0;

    let mut new_grid = Grid::random(24, 24);

    'game_loop: loop {
        new_grid.next_state().render();

        println!("Generation: {}", current_generation);
        println!("\nhit ctrl-c to exit\n");

        if current_generation == MAX_GENERATIONS {
            break 'game_loop;
        }
        current_generation += 1;
        // delay printing to console
        std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY));
    }
}
