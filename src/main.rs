use hmg::{Position, Move, MoveList};


fn main() {

    println!("Positions");
    
    let mut start_position = Position::from_start_position();
    println!("{}", start_position);

    let moves: MoveList = start_position.generate_moves();
    
    println!("{}", moves.len());

    for m in &moves{
        print!("{} ", m);
    }

    println!();
    
}
