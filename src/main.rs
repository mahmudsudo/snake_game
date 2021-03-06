extern crate rand;
extern crate piston_window;
mod draw;
mod game;
mod snake;
use piston_window::{*,types::Color};
use game::Game;
use draw::to_coord_u32;
const BACK_COLOR :Color = [0.5,0.5,0.5,1.0];

fn main() {
    let (w,h) = (30,30);
    let mut window:PistonWindow =WindowSettings::new(
        "Snake",[to_coord_u32(w),to_coord_u32(h)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(w,h);
    while let Some(event) =window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }
        window.draw_2d(&event,|c,g,_|{
            clear(BACK_COLOR,g);
            game.draw(&c,g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
