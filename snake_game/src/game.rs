use core::f64;
use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};

const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];

pub struct Game {
    food_exist: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            food_exist: false,
            food_x: 0,
            food_y: 0,
            width,
            height
        }
    }
    pub fn draw(&self, context: &Context, g: &mut G2d) {
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, context, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g);
        draw_rectangle(BORDER_COLOR, self.width-1, 0, 1, self.height, context, g); 
    }

    pub fn update(&mut self, _delta_time: f64) {
        if !self.food_exist {
            self.add_food();
        }
    }

    pub fn add_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..self.width - 1);
        let y = rng.gen_range(1..self.height - 1);

        self.food_x = x;
        self.food_y = y;
        self.food_exist = true;
    }
}