mod enemy;
mod event;
mod game;
mod item;
mod map;
mod player;
mod serialize;
mod tiles;
mod ui;
mod vector;

use game::{EngineParams, Game};
use map::Map;
use player::Player;
use serialize::Serialize;

fn main() {
    let map = Map::load_from_file("data/Maps/Test.map").expect("Map loading failed!");
    let mut player = Player::new("Paweł".to_string(), 10, 10, 5, 4, 7, 6);
    player.set_position(map.player_position().unwrap());

    let engine_params = EngineParams::new(80, 40, 10);

    let mut game = Game::new(engine_params, player, map);
    while game.is_running() {
        game.handle_input();
        game.render();
    }
}
