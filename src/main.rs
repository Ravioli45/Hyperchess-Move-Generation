use hmg::{diag_relevant_blockers, generate_retractor_captures, orth_relevant_blockers, BUDDY_THROWAWAY, DIAG_THROWAWAY, ORTH_LOOKUPS, RETRACTOR_LOOKUPS, RETRACTOR_MAGICS, STRADLER_LOOKUPS, STRADLER_MAGICS};
use hmg::{generate_diagonal_moves, generate_orthogonal_moves, ORTH_THROWAWAY, ORTH_OFFSETS, DIAG_OFFSETS, RETRACTOR_OFFSETS};
use hmg::{magic_index, ORTH_MAGICS, KING_MOVE_MASK, relevant_buddies, generate_stradler_captures, STRADLER_OFFSETS, RETRACTOR_THROWAWAY};
use hmg::{Bitboard, Square, DEATH_SQUARE_LOOKUP, SPRINGER_CAPTURE_LOOKUP};

fn main() {
    
    for i in 0..64{
        //println!("{:?}", orth_relevant_blockers[i]);
        //println!("{:?}", KING_MOVE_MASK[i]);
        println!("{:?}", relevant_buddies[i]);
    }
    //println!("{:?}", orth_relevant_blockers[27]);
    //for i in 0..64{
    //    println!("{:?}", diag_relevant_blockers[i]);
    //}

    let test_bb = Bitboard(1 << 10 | 1 << 16 | 1 << 32 | 1 << 42 | 1 << 9 | 1 << 4);
    println!("{:?}", test_bb);
    //println!("{:?}", generate_diagonal_moves(Square::A1, test_bb));
    //println!("{:?}", generate_orthogonal_moves(Square::A1, test_bb));
    let mut offset: usize = 0;
    for i in 0..8{
        for j in 0..8{
            //print!("{} ", ORTH_THROWAWAY[i*8 + j]);
            print!("{} ", RETRACTOR_THROWAWAY[i*8 + j]);
        }
        println!("")
    }
    println!("{offset}");
    //println!("{DIAG_OFFSETS:?}");
    println!("{STRADLER_OFFSETS:?}");
    println!("{RETRACTOR_OFFSETS:?}");

    println!("ORTH moves");
    let _square = Square::C3;
    let blockers = test_bb & orth_relevant_blockers[_square];
    let magic_moves = ORTH_LOOKUPS[magic_index(blockers, ORTH_MAGICS[_square], ORTH_THROWAWAY[_square]) + ORTH_OFFSETS[_square]]/* &! test_bb*/;
    println!("{test_bb:?}");
    println!("{:?}", generate_orthogonal_moves(_square, test_bb));
    println!("{magic_moves:?}");
    println!("{:?}", magic_moves &! test_bb);

    let _square2 = Square::C4;
    
    let potential_captures: Bitboard = generate_stradler_captures(_square2, test_bb);
    println!("{:?}", potential_captures);
    let magic_captures = STRADLER_LOOKUPS[magic_index(blockers, STRADLER_MAGICS[_square2], BUDDY_THROWAWAY[_square2]) + STRADLER_OFFSETS[_square2]];
    println!("{magic_captures:?}");

    println!("Retractor");
    let _square3 = Square::B3;

    println!("{:?}", test_bb);
    let r_masked = test_bb & KING_MOVE_MASK[_square3];
    let potential_captures: Bitboard = generate_retractor_captures(_square3, test_bb);
    println!("{:?}", potential_captures);
    let r_magic_captures = RETRACTOR_LOOKUPS[magic_index(r_masked, RETRACTOR_MAGICS[_square3], RETRACTOR_THROWAWAY[_square3]) + RETRACTOR_OFFSETS[_square3]];
    println!("{:?}", r_magic_captures);


    println!("Springer");
    let _square4 = Square::D4;
    let spring_bb = Bitboard(1 << 11 | 1 << 24 | 1 << 29 | 1 << 30 | 1 << 51 | 1 << 59);
    let blockers = spring_bb & orth_relevant_blockers[_square4];
    //let spring_bb = Bitboard::EMPTY;
    println!("{:?}", spring_bb);
    //println!("{:?}", orth_relevant_blockers[_square4]);
    //println!("{:?}", blockers);
    //println!("{:?}", generate_orth_springer_captures(_square4, spring_bb));
    //println!("{:?}", generate_orth_springer_captured(_square4, spring_bb));
    //let magic_result = SPRINGER_ORTH_LOOKUPS[magic_index(blockers, ORTH_MAGICS[_square4], ORTH_THROWAWAY[_square4]) + ORTH_OFFSETS[_square4]];
    //println!("{:?}", magic_result);
    //println!("{:?}", !spring_bb);
    //let final_result = magic_result & !spring_bb;
    //println!("{:?}", magic_result & !spring_bb);

    println!("{:?}", SPRINGER_CAPTURE_LOOKUP[0][7]);

    println!("Death square");
    println!("{:?}", Bitboard(1 << (Square::C3 as u64) |  1 << (Square::G6 as u64)));
    println!("{:?}", DEATH_SQUARE_LOOKUP[Square::C3][Square::G6]);

}
