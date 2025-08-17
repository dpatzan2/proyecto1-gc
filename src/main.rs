mod level;
mod player;
mod events;
mod cast;
mod renderer;
mod texture;
mod framebuffer;

mod game;

fn main() {
    let mut game = game::Game::new().expect("Failed to init game");
    game.run();
}
