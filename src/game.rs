use crate::{
    enemy::Enemy,
    event::{Event, EventType},
    item::Item,
    map::Map,
    player::Player,
    ui::{self, EventWindow, SimplePlayerScr, Size, TextScr},
    vector::Vec2,
};
use console_engine::{pixel, ConsoleEngine, KeyCode};

#[derive(Debug)]
enum GameState {
    Normal,
    Decision,
    Battle,
    Inventory,
}

pub struct Game {
    player: Player,
    engine: ConsoleEngine,
    map: Map,
    event: Event,
    event_window: Option<EventWindow>,
    enemy: Option<Enemy>,
    //item: Option<Item>,
    state: GameState,
    is_running: bool,
}

pub struct EngineParams {
    width: u32,
    height: u32,
    fps: u32,
}

impl EngineParams {
    pub const MIN_WIDTH: u32 = 40;
    pub const MIN_HEIGHT: u32 = 20;
    pub const MIN_FPS: u32 = 4;

    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        Self {
            width: width.max(Self::MIN_WIDTH),
            height: height.max(Self::MIN_HEIGHT),
            fps: fps.max(Self::MIN_FPS),
        }
    }
}

impl Game {
    pub fn new(engine_params: EngineParams, player: Player, map: Map) -> Self {
        let mut engine =
            ConsoleEngine::init(engine_params.width, engine_params.height, engine_params.fps);
        engine.set_title("Text Adventure");
        //let mut event = Event::new();
        //event.player_moved(map.player_position());

        Self {
            player,
            engine,
            map,
            event: Event::new(),
            event_window: None,
            enemy: None,
            //item: None,
            state: GameState::Normal,
            is_running: true,
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.engine.wait_frame();
        self.is_running
    }

    pub fn handle_input(&mut self) {
        match &self.state {
            GameState::Normal => {
                let direction = if self.engine.is_key_pressed(KeyCode::Char('q')) {
                    self.is_running = false;
                    return;
                } else {
                    self.get_input_vector()
                };

                if direction == Vec2::ZERO || !self.map.can_move(self.player.position() + direction)
                {
                    return;
                }

                self.enemy = None;

                self.player.move_vec(direction);
                self.event.player_moved(self.player.position());

                if let Some(event) = self.event.get_event() {
                    match event {
                        EventType::Item(item) => {
                            let event_window = EventWindow::new(
                                Size::new(50, 5),
                                format!("{}", item),
                                "You found item".to_string(),
                                vec!["OK".to_string()],
                            );

                            self.event_window = Some(event_window);
                            self.player.add_to_inventory(item);
                        }
                        EventType::Enemy(enemy) => {
                            self.enemy = Some(enemy);
                            let event_window = EventWindow::new(
                                Size::new(50, 5),
                                format!("{}", self.enemy.as_ref().unwrap()),
                                "You meet enemy".to_string(),
                                vec!["Fight".to_string(), "Ignore".to_string()],
                            );

                            self.event_window = Some(event_window);
                            self.state = GameState::Decision
                        }
                    }
                }
            }

            GameState::Decision => {
                let mut event_window = self.event_window.take().unwrap();
                event_window.handle_input(&self.engine);
                if self.engine.is_key_pressed(KeyCode::Enter) {
                    if event_window.selected() == 0 {
                        self.state = GameState::Battle;
                    } else {
                        self.state = GameState::Normal
                    }
                } else {
                    self.event_window = Some(event_window)
                }
            }
            GameState::Battle => (),
            GameState::Inventory => (),
        }
    }

    pub fn render(&mut self) {
        let width = self.engine.get_width();
        let height = self.engine.get_height();
        self.engine
            .print_screen(0, 0, &ui::main_layout_screen(width, height));

        self.engine.print(61, 34, &format!("GS: {:?}", self.state));

        match &self.state {
            GameState::Normal => self.player_details_renderer(),
            GameState::Decision => {
                self.engine
                    .print_screen(4, 19, &self.event_window.as_ref().unwrap().render())
            }
            GameState::Battle => (),
            GameState::Inventory => (),
        }

        self.engine.draw();
    }

    fn get_input_vector(&self) -> Vec2 {
        if self.engine.is_key_held(KeyCode::Up) {
            Vec2::UP
        } else if self.engine.is_key_held(KeyCode::Down) {
            Vec2::DOWN
        } else if self.engine.is_key_held(KeyCode::Left) {
            Vec2::LEFT
        } else if self.engine.is_key_held(KeyCode::Right) {
            Vec2::RIGHT
        } else {
            Vec2::ZERO
        }
    }

    fn player_details_renderer(&mut self) {
        let mut simple_player_scr = SimplePlayerScr::new(19, 10, &self.player);
        self.engine.print_screen(0, 0, self.map.screen());
        self.engine.set_pxl(
            self.player.position().x,
            self.player.position().y,
            pixel::pxl(Player::SYMBOL),
        );

        self.engine.print_screen(61, 0, simple_player_scr.screen());
    }

    fn item_event_render(&mut self, item: &Item) {
        let contents = vec![
            "You found:".to_string(),
            item.to_string(),
            "Press Enter".to_string(),
        ];
        let item_screen = TextScr::new(20, 4, contents);
        self.engine.print_screen(61, 36, item_screen.screen());
    }

    fn enemy_event_renderer(&mut self, enemy: &Enemy) {
        let contents = vec!["You meet:".to_string(), enemy.to_string()];
        let enemy_info = TextScr::new(20, 4, contents);
        self.engine.print_screen(61, 36, enemy_info.screen());
    }
}
