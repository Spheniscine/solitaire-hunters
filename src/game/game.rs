use std::time::Duration;

use crate::game::Card;

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

pub const ANIMAL_RANK_START: u8 = 6;

impl Card {
    pub fn is_animal(self) -> bool {
        self.rank >= ANIMAL_RANK_START
    }

    pub fn is_hunter(self) -> bool {
        !self.is_animal()
    }
}