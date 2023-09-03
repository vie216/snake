use macroquad::{prelude::*, rand::gen_range};
use snake::Snake;

mod snake;

const WINDOW_SIZE: i32 = 800;
const CELLS: i32 = 24;
const CELL_SIZE: f32 = WINDOW_SIZE as f32 / CELLS as f32;
const UPDATE_DELAY: f32 = 1.0 / 7.0;

#[macroquad::main(config)]
async fn main() {
    let mut state = GameState::new();

    loop {
        state.update();
        state.draw();

        next_frame().await
    }
}

enum GameState {
    Main { snake: Snake, timer: f32 },
    GameOver,
}

impl GameState {
    fn new() -> Self {
        Self::Main {
            snake: Snake::new(),
            timer: 0.0,
        }
    }

    fn update(&mut self) {
        match self {
            Self::Main { snake, timer } => {
                snake.update();

                *timer += get_frame_time();
                if *timer >= UPDATE_DELAY {
                    *timer = 0.0;
                    if !snake.fixed_update() {
                        *self = Self::GameOver;
                    }
                }
            },
            Self::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    *self = Self::new();
                }
            },
        }
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(36, 39, 58, 255));

        match self {
            Self::Main { snake, .. } => snake.draw(),
            Self::GameOver => {
                draw_text_centered("Game Over", 48.0, 0.0, 0.0);
                draw_text_centered("Press Enter to restart", 48.0, 0.0, 48.0);
            },
        }
    }
}

fn config() -> Conf {
    Conf {
        window_title: "Snake!".to_string(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

fn gen_apple() -> IVec2 {
    ivec2(gen_range(0, CELLS), gen_range(0, CELLS))
}

fn draw_text_centered(text: &str, size: f32, offset_x: f32, offset_y: f32) {
    let metrics = measure_text(text, None, size as u16, 1.0);
    draw_text(
        text,
        (screen_width() - metrics.width) / 2.0 + offset_x,
        (screen_height() - metrics.height) / 2.0 + offset_y,
        size,
        Color::from_rgba(202, 211, 245, 255),
    );
}
