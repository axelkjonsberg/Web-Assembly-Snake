use crate::fruit::Fruit;
use crate::game_status::GameStatus;
use crate::point::Point;
use crate::snake::Snake;
use crate::wasm4;
use fastrand::Rng;

// https://wasm4.org/docs/tutorials/snake/placing-the-fruit/

const UPDATE_INTERVAL_IN_TICKS: u32 = 2;
const INITIAL_SNAKE_DIRECTION: Point = Point { x: 1, y: 0 };
const INITIAL_SNAKE_BODY: [Point; 6] = [
    Point { x: 5, y: 0 },
    Point { x: 4, y: 0 },
    Point { x: 3, y: 0 },
    Point { x: 2, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 0 },
];
const GRID_SEGMENT_SIZE: i32 = 4; // 2 * 2 pixels per grid cell
const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 40;

pub struct GameState {
    snake_body: Vec<Point>,
    snake_direction: Point,
    score: u32,
    fruit_position: Point,
}

impl GameState {
    pub fn new(initial_fruit_position: Point) -> Self {
        Self {
            snake_body: INITIAL_SNAKE_BODY.to_vec(),
            snake_direction: INITIAL_SNAKE_DIRECTION,
            score: 0,
            fruit_position: initial_fruit_position,
        }
    }
}

pub struct Game {
    rng: Rng,
    state: GameState,
    status: GameStatus,
    tick_count: u32,
    player_button: u8,
}

impl Game {
    pub fn new(random_seed: u64) -> Self {
        let mut rng = Rng::with_seed(random_seed);
        let initial_fruit_position = Fruit::generate_position(
            &INITIAL_SNAKE_BODY.to_vec(),
            GRID_WIDTH,
            GRID_HEIGHT,
            &mut rng,
        );
        let state = GameState::new(initial_fruit_position);

        Self {
            rng,
            state,
            status: GameStatus::Ongoing,
            tick_count: 0,
            player_button: 0,
        }
    }

    pub fn update(&mut self) {
        match self.status {
            GameStatus::Ongoing => {
                self.tick_count += 1;
                self.poll_player_input();

                if self.tick_count % UPDATE_INTERVAL_IN_TICKS == 0 {
                    self.move_snake();
                }

                if self.is_snake_crashing() {
                    self.status = GameStatus::Finished
                }

                if self.is_snake_eating_fruit() {
                    self.increment_score();
                    self.set_new_fruit_position();
                }

                self.draw_frame();
            }
            GameStatus::Paused => {
                // Maybe some specific logic for when the game is paused
            }
            GameStatus::Finished => {
                self.draw_frame();
            }
        }
    }

    fn poll_player_input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        self.player_button = gamepad;
    }

    fn set_new_fruit_position(&mut self) {
        self.state.fruit_position = Fruit::generate_position(
            &self.state.snake_body,
            GRID_WIDTH,
            GRID_HEIGHT,
            &mut self.rng,
        );
    }

    fn is_snake_eating_fruit(&mut self) -> bool {
        self.state.snake_body.get(0) == Some(&self.state.fruit_position)
    }

    fn is_snake_crashing(&mut self) -> bool {
        Snake::is_overlapping_itself(&self.state.snake_body)
    }

    pub fn move_snake(&mut self) {
        if self.player_button & wasm4::BUTTON_LEFT != 0 {
            Snake::left(&mut self.state.snake_direction);
        }

        if self.player_button & wasm4::BUTTON_RIGHT != 0 {
            Snake::right(&mut self.state.snake_direction);
        }

        if self.player_button & wasm4::BUTTON_UP != 0 {
            Snake::up(&mut self.state.snake_direction);
        }

        if self.player_button & wasm4::BUTTON_DOWN != 0 {
            Snake::down(&mut self.state.snake_direction);
        }

        Snake::move_body(
            &mut self.state.snake_body,
            self.state.snake_direction,
            GRID_WIDTH,
            GRID_HEIGHT,
        );
    }

    fn draw_frame(&self) {
        Snake::draw(&self.state.snake_body, GRID_SEGMENT_SIZE);
        Fruit::draw(self.state.fruit_position, GRID_SEGMENT_SIZE)
    }

    pub fn increment_score(&mut self) {
        self.state.score += 1;
        Snake::grow(
            &mut self.state.snake_body,
            self.state.snake_direction,
            GRID_WIDTH,
            GRID_HEIGHT,
        );
    }
}
