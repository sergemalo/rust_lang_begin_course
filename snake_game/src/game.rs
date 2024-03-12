use core::f64;
use piston_window::types::Color;
use piston_window::*;

use rand::Rng;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    
    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    waiting_time: f64,
    game_over: bool
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exist: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            waiting_time: 0.0,
            game_over: false
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }
    pub fn draw(&self, context: &Context, g: &mut G2d) {
        self.snake.draw(context, g);
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(BORDER_COLOR, self.width-1, 0, 1, self.height, context, g); 
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, context, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exist {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);            
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_tail();
        }        
    }

    fn add_food(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(1..self.width - 1);
        let mut y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(x, y) {
            x = rng.gen_range(1..self.width - 1);
            y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = x;
        self.food_y = y;
        self.food_exist = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.game_over = false;
        self.waiting_time = 0.0;
        self.food_exist = false;
        self.food_x = 0;
        self.food_y = 0;
    }
}