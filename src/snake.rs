use crate::*;

const DIR_RIGHT: Vec2 = vec2(1.0, 0.0);
const DIR_LEFT: Vec2 = vec2(-1.0, 0.0);
const DIR_UP: Vec2 = vec2(0.0, 1.0);
const DIR_DOWN: Vec2 = vec2(0.0, -1.0);

pub struct Snake {
    parts: Vec<Part>,
    direction: Vec2,
    tail_growth_reserve: f32,
}

impl Snake {
    pub fn handle_input(&mut self) {
        let new_direction = if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            DIR_RIGHT
        } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            DIR_LEFT
        } else if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            DIR_DOWN
        } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            DIR_UP
        } else {
            return;
        };

        if self.direction == new_direction || self.direction + new_direction == Vec2::ZERO {
            return;
        }

        let new_head = Part::new(self.head().pos, -new_direction, 0.0);

        self.parts.insert(0, new_head);
        self.direction = new_direction;
    }

    pub fn update(&mut self) {
        let direction = self.direction;
        self.head_mut().pos += direction * SNAKE_SPEED;

        let delta = self.tail_growth_reserve * SNAKE_SPEED * get_frame_time();
        self.tail_growth_reserve -= delta;
        self.tail_mut().len += delta;

        if self.parts.len() > 1 {
            self.head_mut().len += SNAKE_SPEED;
            self.tail_mut().len -= SNAKE_SPEED;

            if self.tail().len <= 0.0 {
                self.parts.pop();
            }
        }
    }

    pub fn draw(&self) {
        for (i, part) in self.parts.iter().enumerate() {
            let part_end = part.end();
            let color = if i % 2 == 0 { COLOR_EVEN } else { COLOR_ODD };

            draw_circle(part.pos.x, part.pos.y, SNAKE_WIDTH / 2.0, color);
            draw_line(
                part.pos.x,
                part.pos.y,
                part_end.x,
                part_end.y,
                SNAKE_WIDTH,
                color,
            );

            if part == self.tail() {
                draw_circle(part_end.x, part_end.y, SNAKE_WIDTH / 2.0, color);
            }
        }
    }

    pub fn grow(&mut self) {
        self.tail_growth_reserve += SNAKE_GROW_AMOUNT;
    }

    pub fn dead(&self) -> bool {
        let head = self.head().pos;

        if head.x < 0.0 || head.x > WINDOW_SIZE || head.y < 0.0 || head.y > WINDOW_SIZE {
            return true;
        }

        for part in &self.parts {
            let part_end = part.end();

            let min_x = part.pos.x.min(part_end.x);
            let max_x = part.pos.x.max(part_end.x);

            if head.x > min_x && head.x < max_x && (head.y - part.pos.y).abs() <= SNAKE_WIDTH {
                return true;
            }

            let min_y = part.pos.y.min(part_end.y);
            let max_y = part.pos.y.max(part_end.y);

            if head.y > min_y && head.y < max_y && (head.x - part.pos.x).abs() <= SNAKE_WIDTH {
                return true;
            }
        }

        false
    }

    pub fn hits_apple(&self, apple: Vec2) -> bool {
        (self.head().pos - apple).abs().length() < SNAKE_WIDTH * 1.5
    }

    fn head(&self) -> &Part {
        &self.parts[0]
    }

    fn head_mut(&mut self) -> &mut Part {
        &mut self.parts[0]
    }

    fn tail(&self) -> &Part {
        &self.parts[self.parts.len() - 1]
    }

    fn tail_mut(&mut self) -> &mut Part {
        let i = self.parts.len() - 1;
        &mut self.parts[i]
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            parts: vec![Part::new(Vec2::ONE * 15.0, DIR_LEFT, 0.0)],
            direction: DIR_RIGHT,
            tail_growth_reserve: 65.0,
        }
    }
}

#[derive(PartialEq)]
struct Part {
    pos: Vec2,
    dir: Vec2,
    len: f32,
}

impl Part {
    fn new(pos: Vec2, dir: Vec2, len: f32) -> Self {
        Self { pos, dir, len }
    }

    fn end(&self) -> Vec2 {
        self.pos + self.dir * self.len
    }
}
