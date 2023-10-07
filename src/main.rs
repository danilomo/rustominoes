mod rustominoes;

fn main() {
    let _game = rustominoes::Game::new(4);

    let a = rustominoes::Domino(1, 2);
    let b = a.match_left(2);
    let c = a.match_right(2);
    let d = a.match_right(3);

    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}
