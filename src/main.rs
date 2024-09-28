extern crate shakmaty;

use shakmaty::{Chess, Position};

mod random_ai;
mod ai;

use random_ai::*;
use ai::*;

fn main() {
    println!("Hello, Gamer!");

    let pos : Chess = Chess::default();
    println!("{}", pos.board());
    let rai = Random_Ai{};
    let mov_opt = rai.gen_move(&pos);

    if let Some(mov) = mov_opt {
        if let Ok(pos) = pos.play(&mov) {
            println!("Played our first move!");
            println!("{}", pos.board());
        } else {
            println!("Failed to play our first move!");
        }

    }else {
        println!("Failed to play our first move!");
    }
}

