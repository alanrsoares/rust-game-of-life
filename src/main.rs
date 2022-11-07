use game_of_life::Game;
use game_of_life::Grid;

fn main() {
    const GRID_SIZE: i32 = 20;
    const MAX_GENERATIONS: usize = 100;
    const FRAME_DELAY: u64 = (1000 / 60) * 6;

    let mut game = Game::new(
        Grid::random(GRID_SIZE, GRID_SIZE),
        MAX_GENERATIONS,
        FRAME_DELAY,
    );

    game.run();
}
