use life::Game;
use life::Grid;

const GRID_SIZE: i32 = 20;
const MAX_GENERATIONS: usize = 100;
const FRAME_DELAY: u64 = (1000 / 60) * 6;

fn main() {
    let mut game = Game::new(
        Grid::random(GRID_SIZE, GRID_SIZE),
        MAX_GENERATIONS,
        FRAME_DELAY,
    );

    game.run();
}
