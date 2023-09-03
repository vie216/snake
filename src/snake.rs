use crate::*;

pub struct Snake {
    cells: Vec<IVec2>,
    direction: Direction,
    direction_queue: Vec<Direction>,
    apple: IVec2,
    score: usize,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            cells: vec![ivec2(0, 0)],
            direction: Direction::Right,
            direction_queue: Vec::new(),
            apple: gen_apple(),
            score: 0,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            self.direction_queue.push(Direction::Right);
        } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            self.direction_queue.push(Direction::Left);
        } else if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            self.direction_queue.push(Direction::Down);
        } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.direction_queue.push(Direction::Up);
        }
    }

    pub fn fixed_update(&mut self) -> bool {
        while !self.direction_queue.is_empty() {
            let new = self.direction_queue.remove(0);

            if self.direction.can_change_to(new) {
                self.direction = new;
                break;
            }
        }

        for i in (0..self.cells.len()).rev() {
            match i {
                0 => self.cells[0] += self.direction.to_ivec2(),
                _ => {
                    if self.cells[i] != self.cells[i - 1] {
                        self.cells[i] = self.cells[i - 1];
                    }
                },
            }

            let cell = &mut self.cells[i];

            if cell.x < 0 {
                cell.x = CELLS - 1;
            } else if cell.x >= CELLS {
                cell.x = 0;
            }

            if cell.y < 0 {
                cell.y = CELLS - 1;
            } else if cell.y >= CELLS {
                cell.y = 0;
            }
        }

        if self.cells[0] == self.apple {
            self.apple = gen_apple();
            self.score += 1;

            let last = self.cells.last().unwrap();
            self.cells.push(*last);
        } else {
            for i in 1..self.cells.len() {
                if self.cells[i] == self.cells[0] {
                    return false;
                }
            }
        }

        true
    }

    pub fn draw(&self) {
        for i in 0..self.cells.len() {
            let color = if i == 0 {
                Color::from_rgba(238, 212, 159, 255)
            } else {
                if i % 2 == 0 {
                    Color::from_rgba(244, 219, 214, 255)
                } else {
                    Color::from_rgba(183, 189, 248, 255)
                }
            };

            draw_rectangle(
                self.cells[i].x as f32 * CELL_SIZE,
                self.cells[i].y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }

        draw_rectangle(
            self.apple.x as f32 * CELL_SIZE,
            self.apple.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            Color::from_rgba(237, 135, 150, 255),
        );

        draw_text(
            &self.score.to_string(),
            20.0,
            WINDOW_SIZE as f32 - 20.0,
            72.0,
            Color::from_rgba(202, 211, 245, 255),
        );
    }
}

#[derive(Default, Clone, Copy)]
enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn to_ivec2(self) -> IVec2 {
        match self {
            Self::Right => ivec2(1, 0),
            Self::Left => ivec2(-1, 0),
            Self::Up => ivec2(0, 1),
            Self::Down => ivec2(0, -1),
        }
    }

    fn can_change_to(&self, new: Self) -> bool {
        ((*self as isize) < 2) ^ ((new as isize) < 2)
    }
}
