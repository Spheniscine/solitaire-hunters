use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, NUM_SUITS, RANK_MAX, RANK_MIN, RANKS, SettingsState, Skin, Suit}};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

pub const ANIMAL_RANK_START: u8 = 6;
pub const ANIMAL_COPIES: usize = (RANK_MAX + 1 - ANIMAL_RANK_START) as usize;
pub const NUM_HUNTER_RANKS: usize = (ANIMAL_RANK_START - RANK_MIN) as usize;

impl Card {
    pub fn is_animal(self) -> bool {
        self.rank >= ANIMAL_RANK_START
    }

    pub fn is_hunter(self) -> bool {
        !self.is_animal()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    #[serde(skip)]
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        self.check_auto_moves();

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        if back.is_animal() {
            front.is_animal() && back.suit == front.suit
        } else {
            front.is_hunter() && back.suit.color() != front.suit.color() && front.rank + 1 == back.rank
        }
    }

    fn is_stack(&self, slice: &[Card]) -> bool {
        slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
    }

    pub fn is_won(&self) -> bool {
        DepotRole::Shadow.range().filter(|&s| {
            self.board.depots[s].len() == ANIMAL_COPIES
        }).count() == NUM_SUITS &&
        DepotRole::Tableau.range().filter(|&s| {
            let depot = &self.board.depots[s];
            depot.len() == NUM_HUNTER_RANKS && self.is_stack(depot)
        }).count() == NUM_SUITS
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::Tableau => { self.is_stack(slice) },
            DepotRole::Shadow => false,
            DepotRole::FreeCell => { slice.len() <= 1 },
        }
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_won() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos::new(pos.depot_index, pos.card_index.wrapping_add(1));
            if !self.can_move(src, dest) { return; }
            self.undo_stack.push(self.history.len());
            self.do_move_raw(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn ondoubleclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_won() { return; }
        if !self.can_select(pos) { return; } // needed, or illegal stacks can still be moved this way!

        let dest = DepotRole::FreeCell.id(0);
        let dest = BoardPos::new(dest, self.board.depots[dest].len());
        if self.can_move(pos, dest) {
            self.undo_stack.push(self.history.len());
            self.do_move_raw(pos, dest);
        }
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord { pos1, pos2 })
    }

    pub fn can_move(&self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];
        let Some((role, ix)) = DepotRole::role_and_subindex(pos2.depot_index) else { return false };

        match role {
            DepotRole::Tableau => {
                self.board.depots[DepotRole::Shadow.id(ix)].is_empty() &&
                depot2.last().is_none_or(|&c| self.can_stack(c, card))
            },
            DepotRole::Shadow => false,
            DepotRole::FreeCell => {
                depot2.is_empty() && num_moved == 1
            },
        }
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            self.board.do_move(rec.pos2, rec.pos1);
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();

        self.check_auto_moves();
        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn check_auto_moves(&mut self) {
        if self.is_busy() { return; }
        
        // check for full animal stacks and move them to shadow zones simultaneously
        for i in 0..DepotRole::Tableau.number_of() {
            let cards = &self.board.depots[DepotRole::Tableau.id(i)];
            if cards.len() == ANIMAL_COPIES && cards[0].is_animal() && self.is_stack(&cards) {
                let src = BoardPos::new(DepotRole::Tableau.id(i), 0);
                let dest = self.board.top_pos(DepotRole::Shadow.id(i));
                self.do_move_raw(src, dest);
            }
        }
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            allow_undo: self.allow_undo,
            skin: self.skin,
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState){
        self.allow_undo = settings.allow_undo;
        self.skin = settings.skin;
        LocalStorage.save_game_state(&self);
    }
}