use super::solve::solve;
use super::tiles::{Tiles, TilesError};
use super::pile::*;

pub fn main() -> Result<(), TilesError> {
    if std::env::args().count() < 2 {
        println!("You need to specify some arguments");
        let pile = Pile::new();
        pile.count();
        println!("{}", pile.pile);
    } else {
        for arg in std::env::args().skip(1) {
            let tiles = arg.parse::<Tiles>()?;
            println!("Trying to solve board: {}", tiles);
            for solution in solve(tiles) {
                println!("Solution: {}", solution);
            }
            println!("* * *");
        }
    }

    Ok(())
}
