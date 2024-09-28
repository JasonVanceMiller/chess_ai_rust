extern crate shakmaty;

use shakmaty::*;

pub trait Ai {
    fn gen_move(&self, pos: &Chess) -> Option<Move>; 
}
