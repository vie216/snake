use macroquad::prelude::*;
use snake::Snake;
use std::time::SystemTime;

mod snake;

const WINDOW_SIZE: f32 = 800.0;
const SNAKE_WIDTH: f32 = 15.0;
const SNAKE_SPEED: f32 = 5.0;
const SNAKE_GROW_AMOUNT: f32 = 50.0;

const COLOR_BG: Color = color_u8!(36, 39, 58, 255);
const COLOR_EVEN: Color = color_u8!(244, 219, 214, 255);
const COLOR_ODD: Color = color_u8!(183, 189, 248, 255);
const COLOR_APPLE: Color = color_u8!(237, 135, 150, 255);
const COLOR_TEXT: Color = color_u8!(202, 211, 245, 255);

#[macroquad::main(config)]
async fn main() {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards!");
    rand::srand(time.as_secs());

    let mut state = GameState::default();

    loop {
        clear_background(COLOR_BG);

        match &mut state {
            GameState::Main {
                snake,
                apple,
                score,
            } => {
                snake.handle_input();
                snake.update();
                snake.draw();

                if snake.dead() {
                    state = GameState::GameOver { score: *score };
                    continue;
                }

                if snake.hits_apple(*apple) {
                    *apple = gen_apple();
                    *score += 1;
                    snake.grow();
                }

                draw_circle(apple.x, apple.y, SNAKE_WIDTH, COLOR_APPLE);
                draw_text(
                    &score.to_string(),
                    20.0,
                    WINDOW_SIZE - 20.0,
                    48.0,
                    COLOR_TEXT,
                );
            },
            GameState::GameOver { score } => {
                if is_key_pressed(KeyCode::Enter) {
                    state = GameState::default();
                    continue;
                }

                draw_text_centered("Game Over", 72.0, 0.0, -22.0);
                draw_text_centered("Press Enter to restart", 48.0, 0.0, 22.0);
                draw_text(
                    &score.to_string(),
                    20.0,
                    WINDOW_SIZE - 20.0,
                    48.0,
                    COLOR_TEXT,
                );
            },
        }

        next_frame().await
    }
}

enum GameState {
    Main {
        snake: Snake,
        apple: Vec2,
        score: usize,
    },
    GameOver {
        score: usize,
    },
}

impl Default for GameState {
    fn default() -> Self {
        Self::Main {
            snake: Snake::default(),
            apple: gen_apple(),
            score: 0,
        }
    }
}

fn config() -> Conf {
    Conf {
        window_title: "Snake!".to_string(),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn gen_apple() -> Vec2 {
    vec2(
        rand::gen_range(SNAKE_WIDTH, WINDOW_SIZE - SNAKE_WIDTH),
        rand::gen_range(SNAKE_WIDTH, WINDOW_SIZE - SNAKE_WIDTH),
    )
}

fn draw_text_centered(text: &str, size: f32, offset_x: f32, offset_y: f32) {
    let metrics = measure_text(text, None, size as u16, 1.0);
    draw_text(
        text,
        (screen_width() - metrics.width) / 2.0 + offset_x,
        (screen_height() - metrics.height) / 2.0 + offset_y,
        size,
        COLOR_TEXT,
    );
}
