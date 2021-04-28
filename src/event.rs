use rand::Rng;

use crate::{enemy::Enemy, item::Item, vector::Vec2};
use std::collections::HashMap;

const BASE_EVENT_CHANCE: u8 = 20;
const ITEM_FIND_CHANCE: u8 = 20;

#[derive(Debug)]
pub enum EventType {
    Item(Item),
    Enemy(Enemy),
}

struct VistDetails {
    count: u8,
    item_found: bool,
    enemy_meet: bool,
}

impl VistDetails {
    fn new() -> Self {
        Self {
            count: 0,
            item_found: false,
            enemy_meet: false,
        }
    }
}

pub struct Event {
    visited: HashMap<Vec2, VistDetails>,
    event_taken: bool,
    event_type: Option<EventType>,
    current_pos: Vec2,
}

impl Event {
    pub fn new() -> Self {
        Self {
            visited: HashMap::new(),
            event_taken: false,
            event_type: None,
            current_pos: Vec2::ZERO,
        }
    }

    pub fn player_moved(&mut self, player_pos: Vec2) {
        let visit = self.visited.entry(player_pos).or_insert(VistDetails::new());
        if visit.count < u8::MAX {
            visit.count += 1;
        }

        self.current_pos = player_pos;
        self.event_taken = false;
    }

    pub fn get_event(&mut self) -> Option<EventType> {
        if self.event_taken {
            return None;
        }

        self.event_taken = true;
        let visit = self.visited.get_mut(&self.current_pos).unwrap();

        if visit.enemy_meet && visit.item_found {
            // This location provided item and enemy. Nothing to see here.
            return None;
        } else {
            let event_chance = if u8::MAX - visit.count < BASE_EVENT_CHANCE {
                255
            } else {
                visit.count + BASE_EVENT_CHANCE
            };

            let mut rng = rand::thread_rng();
            let chance: u8 = rng.gen();
            if chance > event_chance {
                return None;
            }

            if !(visit.enemy_meet || visit.item_found) {
                // This location didn't provided any type of event yet.
                let visit_type: u8 = rng.gen();
                if visit_type > ITEM_FIND_CHANCE {
                    visit.enemy_meet = true;
                    Some(EventType::Enemy(Enemy::new()))
                } else {
                    visit.item_found = true;
                    Some(EventType::Item(Item::new()))
                }
            } else if !visit.enemy_meet {
                // This location didn't have enemy meeting event
                visit.enemy_meet = true;
                Some(EventType::Enemy(Enemy::new()))
            } else {
                // This location didn't have item found event
                visit.item_found = true;
                Some(EventType::Item(Item::new()))
            }
        }
    }

    pub fn in_progress(&self) -> bool {
        self.event_taken
    }
}
