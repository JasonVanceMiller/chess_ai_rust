extern crate shakmaty;

use shakmaty::*;

mod random_ai;
mod ai;

use random_ai::*;
use ai::*;

fn main() {
    println!("Hello, Gamer!");

    let mut pos : Chess = Chess::default();
    let player1 = Random_Ai{};
    let player2 = Random_Ai{};

    while pos.outcome() == None && pos.halfmoves() < 10000 {
        let mov_opt : Option<Move>;
        if pos.turn() == Color::White {
            mov_opt = player1.gen_move(&pos);
        } else {
            mov_opt = player2.gen_move(&pos);
        }

        let Some(mov) = mov_opt else { break };

        let pos_res = pos.clone().play(&mov);
        let Ok(pos_ok) = pos_res else { break };
        pos = pos_ok;
        println!("{}", pos.board());
    }
    match pos.outcome() {
        Some(out) => { println!("{}", out); },
        None      => { println!("Turn Limit Reached."); },
    }

}

