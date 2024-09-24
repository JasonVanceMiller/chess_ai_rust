extern crate shakmaty;

fn main() {
    println!("Hello, world!");

    use shakmaty::{Chess, Position};

    let pos = Chess::default();
    let legals = pos.legal_moves();
    assert_eq!(legals.len(), 20);
}
