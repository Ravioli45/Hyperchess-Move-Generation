use hmg::{diag_relevant_blockers, orth_relevant_blockers, BUDDY_THROWAWAY, DIAG_THROWAWAY, ORTH_LOOKUPS, STRADLER_LOOKUPS, STRADLER_MAGICS};
use hmg::{generate_diagonal_moves, generate_orthogonal_moves, ORTH_THROWAWAY, ORTH_OFFSETS, DIAG_OFFSETS};
use hmg::{magic_index, ORTH_MAGICS, KING_MOVE_MASK, relevant_buddies, generate_stradler_captures, STRADLER_OFFSETS};
use hmg::{Bitboard, Square};

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
            print!("{} ", BUDDY_THROWAWAY[i*8 + j]);
        }
        println!("")
    }
    println!("{offset}");
    //println!("{DIAG_OFFSETS:?}");
    println!("{STRADLER_OFFSETS:?}");

    let _square = Square::C3;
    let blockers = test_bb & orth_relevant_blockers[_square];
    let magic_moves = ORTH_LOOKUPS[magic_index(blockers, ORTH_MAGICS[_square], ORTH_THROWAWAY[_square]) + ORTH_OFFSETS[_square]] &! test_bb;
    println!("{test_bb:?}");
    println!("{magic_moves:?}");

    let _square2 = Square::E6;
    
    let potential_captures: Bitboard = generate_stradler_captures(_square2, test_bb);
    println!("{:?}", potential_captures);
    let magic_captures = STRADLER_LOOKUPS[magic_index(blockers, STRADLER_MAGICS[_square2], BUDDY_THROWAWAY[_square2]) + STRADLER_OFFSETS[_square2]];
    println!("{magic_captures:?}");
}
