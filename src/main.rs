extern crate shakmaty;

use shakmaty::{Chess, Position};

fn main() {
    println!("Hello, Gamer!");

    let pos : Chess = Chess::default();
    println!("{}", pos.board());
    let legals = pos.legal_moves();
    let result = pos.play(&legals[0]);
    if let Ok(pos) = result {
        println!("Played our first move!");
        println!("{}", pos.board());
    } else {
        println!("Failed to play our first move!");
    }
    //assert_eq!(legals.len(), 20);
}

