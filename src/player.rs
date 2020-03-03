use super::tiles::*;
use super::pile::Pile;

pub struct Player {
    pub hand: Vec<Tiles>,
    pub broken_in: bool
}

impl Player {
    fn new() -> Player {
        let mut hand = vec![];
       
        Player { hand:hand, broken_in: false }
    }
}