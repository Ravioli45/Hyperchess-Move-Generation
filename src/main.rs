use std::io::{self, Write};
use hmg::{Position, Move, MoveList};

const MOVES_PER_ROW: usize = 15;

fn main() -> io::Result<()>{
    
    println!("Positions");

    let mut position = Position::from_start_position();

    let mut in_buffer = String::new();
    let mut previous_moves: Vec<Move> = Vec::new();

    loop{
        in_buffer.clear();

        println!("{}", position);
        //println!("{}", position.is_attacking_king());
        //println!("{}, {}", position.is_check(), position.is_checkmate());

        let moves: MoveList = position.generate_moves();

        for (i, m) in moves.iter().enumerate(){
            print!("{}:{} ", i, m);
            if ((i+1) % MOVES_PER_ROW) == 0{
                println!()
            }
        }
        println!();

        print!("Select move by index: ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut in_buffer)?;

        let trimmed: &str = in_buffer.trim();
        
        if trimmed == "q"{
            break;
        }
        else if trimmed == "u"{
            if let Some(m) = previous_moves.pop(){
                position.unmake_move(m);
            }
            continue;
        }

        let Ok(selected) = trimmed.parse::<usize>() else{
            continue;
        };

        let Some(m) = moves.get(selected) else{
            continue;
        };

        //println!("{m:?}");
        position.make_move(m);
        previous_moves.push(m);
        
    }

    Ok(())
}
