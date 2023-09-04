use crate::*;

const DIR_RIGHT: Vec2 = vec2(1.0, 0.0);
const DIR_LEFT: Vec2 = vec2(-1.0, 0.0);
const DIR_UP: Vec2 = vec2(0.0, 1.0);
const DIR_DOWN: Vec2 = vec2(0.0, -1.0);

pub struct Snake {
    head: Vec2,
    direction: Vec2,
    turns: Vec<Turn>,
    len: f32,
    target_len: f32,
    tail_len: f32,
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

        let turn = Turn {
            pos: self.head,
            dir: Vec2::ZERO - self.direction,
        };

        self.direction = new_direction;
        self.turns.insert(0, turn);
    }

    pub fn update(&mut self) {
        self.head += self.direction * SNAKE_SPEED;
        self.len += (self.target_len - self.len) * SNAKE_SPEED * get_frame_time();

        let mut prev = self.head;
        let mut len = self.len;

        for i in 0..self.turns.len() {
            let turn = self.turns[i];
            let diff = (turn.pos - prev).abs();
            let segment_len = diff.x.max(diff.y);

            if segment_len > len {
                self.turns.pop();
                break;
            }

            len -= segment_len;
            prev = turn.pos;
        }

        self.tail_len = len;
    }

    pub fn draw(&self) {
        let mut prev = &Turn {
            pos: self.head,
            dir: Vec2::ZERO - self.direction,
        };

        for (i, turn) in self.turns.iter().enumerate() {
            let color = get_segment_color(i);

            draw_circle(prev.pos.x, prev.pos.y, SNAKE_WIDTH / 2.0, color);
            draw_line(
                prev.pos.x,
                prev.pos.y,
                turn.pos.x,
                turn.pos.y,
                SNAKE_WIDTH,
                color,
            );

            prev = turn;
        }

        let color = get_segment_color(self.turns.len());
        let end = prev.pos + prev.dir * self.tail_len;

        draw_circle(prev.pos.x, prev.pos.y, SNAKE_WIDTH / 2.0, color);
        draw_line(prev.pos.x, prev.pos.y, end.x, end.y, SNAKE_WIDTH, color);
        draw_circle(end.x, end.y, SNAKE_WIDTH / 2.0, color);
    }

    pub fn grow(&mut self) {
        self.target_len += SNAKE_GROW_AMOUNT;
    }

    pub fn dead(&self) -> bool {
        if self.head.x < 0.0
            || self.head.x > WINDOW_SIZE
            || self.head.y < 0.0
            || self.head.y > WINDOW_SIZE
        {
            return true;
        }

        for [prev, cur] in self.turns.windows(2).skip(1) {
            if head_overlaps_with_segment(self.head, prev.pos, cur.pos) {
                return true;
            }
        }

        if !self.turns.is_empty() {
            let prev = self.turns[self.turns.len() - 1];
            let overlaps = head_overlaps_with_segment(
                self.head,
                prev.pos,
                prev.pos + prev.dir * self.tail_len,
            );

            if overlaps {
                return true;
            }
        }

        false
    }

    pub fn hits_apple(&self, apple: Vec2) -> bool {
        (self.head - apple).abs().length() < SNAKE_WIDTH * 1.5
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            head: Vec2::ONE * 15.0,
            direction: DIR_RIGHT,
            turns: Vec::new(),
            len: 0.0,
            target_len: 65.0,
            tail_len: 65.0,
        }
    }
}

struct Turn {
    pos: Vec2,
    dir: Vec2,
}

#[inline]
fn get_segment_color(segment_index: usize) -> Color {
    if segment_index % 2 == 0 {
        return COLOR_EVEN;
    }

    COLOR_ODD
}

#[inline]
fn head_overlaps_with_segment(head: Vec2, segment_start: Vec2, segment_end: Vec2) -> bool {
    let min_x = segment_start.x.min(segment_end.x);
    let max_x = segment_start.x.max(segment_end.x);

    if head.x > min_x && head.x < max_x && (head.y - segment_start.y).abs() <= SNAKE_WIDTH {
        return true;
    }

    let min_y = segment_start.y.min(segment_end.y);
    let max_y = segment_start.y.max(segment_end.y);

    if head.y > min_y && head.y < max_y && (head.x - segment_start.x).abs() <= SNAKE_WIDTH {
        return true;
    }

    false
}
