extern crate piston_window;
extern crate rand;

mod draw;
mod game;

use piston_window::*;
//use crate::game;
//use crate::draw::to_coord;

const BACKGROUND_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new("Snake", 
    [draw::to_coord(width), draw::to_coord(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(width, height);
    
    while let Some(event) = window.next() {
        //if let Some(Button::Keyboard(key)) = event.press_args() {
        //    game.key_pressed(key);
        //}
        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
    
}