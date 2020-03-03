use super::tiles::{Tiles, Color, Tile};

pub struct Pile {
    pub pile: Tiles
}
impl Pile {
    pub fn new() -> Pile {
        // Add in 104 number tiles in game 
        // 2 jokes + 2 of each color and type
        let mut tiles = Tiles::new();
        for tile in Tile::all() {
            tiles.set_count(&tile, 2);
        }
        Pile { pile: tiles }
    }

    pub fn count(&self) -> u32 {
        //println!("#1's: {}", self.pile.count());
        self.pile.print_raw();
        self.pile.get_total_count() as u32
    }

    /*fn get_tile(mut &self) -> Option<Tile> {

    } */   
}
