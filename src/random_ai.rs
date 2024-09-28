extern crate shakmaty;

use shakmaty::*;
use rand::Rng;

use crate::ai::*;

pub struct Random_Ai {

}

impl Ai for Random_Ai {
    fn gen_move(&self, pos : &Chess) -> Option<Move> {
        let legals : MoveList = pos.legal_moves();
        if legals.len() > 0 {
            let num = rand::thread_rng().gen_range(0..legals.len());
            return Some(legals[num].clone());
        } else {
            return None;
        }

    }
}
