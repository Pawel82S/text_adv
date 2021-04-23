use crate::player::Player;
use console_engine::screen::Screen;

const LABEL_COL: i32 = 0;
const VALUE_COL: i32 = 6;

pub struct SimplePlayerScr<'a> {
    player: &'a Player,
    screen: Screen,
}

impl<'a> SimplePlayerScr<'a> {
    pub fn new(width: u32, height: u32, player: &'a Player) -> Self {
        let screen = Screen::new(width, height);
        let mut result = Self { player, screen };
        result.render();
        result
    }

    pub fn screen(&mut self) -> &Screen {
        self.render();
        &self.screen
    }

    fn render(&mut self) {
        // Handy shortcuts
        let p = self.player;
        let s = &mut self.screen;

        // Name
        let mut row = 0;
        s.print(LABEL_COL, row, "Name:");
        s.print(VALUE_COL, row, p.name());

        // Level
        row += 1;
        s.print(LABEL_COL, row, "Lvl:");
        s.print(VALUE_COL, row, &p.level().to_string());

        // Movement
        s.print(VALUE_COL + 3, row, "Mvt:");
        s.print(
            VALUE_COL + 8,
            row,
            &format!("{}/{}", p.remaining_moves(), p.max_moves()),
        );

        // Experience
        row += 1;
        s.print(LABEL_COL, row, "XP:");
        s.print(
            VALUE_COL,
            row,
            &format!("{}/{}", p.current_xp(), p.next_level_xp()),
        );

        // Health
        row += 1;
        s.print(LABEL_COL, row, "Hlth:");
        s.print(
            VALUE_COL,
            row,
            &format!("{}/{}", p.current_health(), p.max_health()),
        );

        // Mana
        row += 1;
        s.print(LABEL_COL, row, "Mana:");
        s.print(
            VALUE_COL,
            row,
            &format!("{}/{}", p.current_mana(), p.max_mana()),
        );

        // Attack
        row += 1;
        s.print(LABEL_COL, row, "Att:");
        s.print(VALUE_COL, row, &format!("{}", p.attack()));

        // Defense
        row += 1;
        s.print(LABEL_COL, row, "Def:");
        s.print(VALUE_COL, row, &format!("{}", p.defense()));

        // Strenght
        row += 2;
        s.print(LABEL_COL, row, "Str:");
        s.print(VALUE_COL, row, &format!("{}", p.strenght()));

        // Intellingence
        s.print(VALUE_COL + 5, row, "Int:");
        s.print(VALUE_COL + 10, row, &format!("{}", p.intelligence()));

        // Dexterity
        row += 1;
        s.print(LABEL_COL, row, "Dex:");
        s.print(VALUE_COL, row, &format!("{}", p.dexterity()));

        // Speed
        s.print(VALUE_COL + 5, row, "Spd:");
        s.print(VALUE_COL + 10, row, &format!("{}", p.speed()));
    }
}
