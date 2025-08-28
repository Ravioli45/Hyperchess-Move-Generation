use crate::types::{Piece, Square};

use std::fmt;
//use std::mem::transmute;
use std::ops::Index;
use std::array::IntoIter;
use std::slice::Iter;
use std::iter::Take;

const MAX_MOVES: usize = 256;

type MoveListIntoIter = Take<IntoIter<Move, MAX_MOVES>>;
type MoveListIter<'a> = Take<Iter<'a, Move>>;

/// move info in 32 bits
/// 
/// 6 bits (0x3f): from
/// 
/// 6 bits (0xFC0): to
/// 
/// 3 bits (0x7000): piece type that moved
///
/// remaining bits describe captures
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(u32);
impl Move{

    pub fn is_capture(&self) -> bool{
        ((self.0 & 0x7FFF8000) >> 15) != 0
    }

    pub(crate) fn get_capture_bits(&self) -> u32{
        (self.0 & 0x7FFF8000) >> 15
    }
    pub(crate) unsafe fn set_capture_bits(&mut self, bits: u32){
        self.0 |= bits << 15;
    }

    pub(crate) fn get_from(&self) -> Square{
        //(self.0 & 0x3f) as usize
        /*
        unsafe{
            transmute::<usize, Square>((self.0 & 0x3f) as usize)
        }
        */
        Square::try_from((self.0 & 0x3f) as usize).unwrap()
    }
    pub(crate) fn set_from(&mut self, s: Square){
        self.0 |= s as u32;
    }

    pub(crate) fn get_to(&self) -> Square{
        //((self.0 & 0xFC0) >> 6) as usize
        /*
        unsafe{
            transmute::<usize, Square>(((self.0 & 0xFC0) >> 6) as usize)
        }
        */
        Square::try_from(((self.0 & 0xFC0) >> 6) as usize).unwrap()
    }
    pub(crate) fn set_to(&mut self, s: Square){
        self.0 |= (s as u32) << 6;
    }

    pub(crate) fn get_piece(&self) -> Piece{
        /*
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x7000) >> 12) as usize)
        }
        */
        Piece::try_from(((self.0 & 0x7000) >> 12) as usize).unwrap()
    }
    pub(crate) fn set_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 12;
    }

    pub(crate) fn get_c1_piece(&self) -> Piece{
        /*
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x38000) >> 15) as usize)
        }
        */
        Piece::try_from(((self.0 & 0x38000) >> 15) as usize).unwrap()
    }
    pub(crate) fn set_c1_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 15;
    }

    pub(crate) fn get_c2_piece(&self) -> Piece{
        /*
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x1C0000) >> 18) as usize)
        }
        */
        Piece::try_from(((self.0 & 0x1C0000) >> 18) as usize).unwrap()
    }
    pub(crate) fn set_c2_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 18;
    }

    pub(crate) fn get_c3_piece(&self) -> Piece{
        /*
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0xE00000) >> 21) as usize)
        }
        */
        Piece::try_from(((self.0 & 0xE00000) >> 21) as usize).unwrap()
    }
    pub(crate) fn set_c3_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 21;
    }

    pub(crate) fn get_c4_piece(&self) -> Piece{
        /*
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x7000000) >> 24) as usize)
        }
        */
        Piece::try_from(((self.0 & 0x7000000) >> 24) as usize).unwrap()
    }
    pub(crate) fn set_c4_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 24;
    }

    pub(crate) fn get_c5_bit(&self) -> bool{
       ((self.0 & 0x8000000) >> 27) != 0
    }
    pub(crate) fn set_c5_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 27;
    }

    pub(crate) fn get_c6_bit(&self) -> bool{
       ((self.0 & 0x10000000) >> 28) != 0
    }
    pub(crate) fn set_c6_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 28;
    }

    pub(crate) fn get_c7_bit(&self) -> bool{
       ((self.0 & 0x20000000) >> 29) != 0
    }
    pub(crate) fn set_c7_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 29;
    }

    pub(crate) fn get_c8_bit(&self) -> bool{
       ((self.0 & 0x40000000) >> 30) != 0
    }
    pub(crate) fn set_c8_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 30;
    }

}
impl Default for Move{
    fn default() -> Self {
        Move(0)
    }
}
impl fmt::Display for Move{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}{}", self.get_from(), self.get_to())
    }
}
impl fmt::Debug for Move{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}, {}, {:b}", self.get_piece() as usize, self, self.get_capture_bits())
    }
}

#[derive(Debug)]
pub struct MoveList{
    moves: [Move; MAX_MOVES],
    size: usize,
}
impl MoveList{
    
    pub fn new() -> Self{
        MoveList{ 
            moves: [Move::default(); MAX_MOVES], 
            size: 0 
        }
    }

    pub const fn len(&self) -> usize{
        self.size
    }

    pub fn get(&self, index: usize) -> Option<Move>{
        if index < self.size{
            Some(self.moves[index])
        }
        else{
            None
        }
    }

    pub fn iter(&self) -> MoveListIter{
        self.moves.iter().take(self.size)
    }

    pub(crate) fn add_move(&mut self, m: Move){
        assert!(self.size <= MAX_MOVES);
        self.moves[self.size] = m;
        self.size += 1;
    }
}
impl Index<usize> for MoveList{
    type Output = Move;

    fn index(&self, index: usize) -> &Move{
        if index < self.size{
            self.moves.index(index)
        }
        else{
            panic!("index out of bounds")
        }
    }
}
impl IntoIterator for MoveList{
    type Item = Move;
    type IntoIter = MoveListIntoIter;

    fn into_iter(self) -> MoveListIntoIter{
        self.moves.into_iter().take(self.size)
    }
}
impl<'a> IntoIterator for &'a MoveList{
    type Item = &'a Move;
    type IntoIter = MoveListIter<'a>;

    fn into_iter(self) -> MoveListIter<'a>{
        self.iter()
    }
}
impl FromIterator<Move> for MoveList{
    fn from_iter<T: IntoIterator<Item = Move>>(iter: T) -> Self{
        let mut result: MoveList = MoveList::new();
        iter.into_iter().for_each(|m| result.add_move(m));
        result
    }
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn simple_move_test(){

        // stradler moves from e2->e4 and captures stradler on f4
        let m: u32 = (Square::E2 as u32) | ((Square::E4 as u32) << 6)
            | ((Piece::Stradler as u32) << 12) | ((Piece::Stradler as u32) << 18);

        let test_move = Move(m);

        assert_eq!(test_move.get_from(), Square::E2);
        assert_eq!(test_move.get_to(), Square::E4);

        assert_eq!(test_move.get_piece(), Piece::Stradler);

        assert_eq!(test_move.get_c2_piece(), Piece::Stradler);
    }

    #[test]
    fn move_construction_test(){
        let mut m = Move(0);

        m.set_from(Square::G1);
        m.set_to(Square::C5);

        m.set_piece(Piece::Springer);
        m.set_c1_piece(Piece::Stradler);


        assert_eq!(m.get_from(), Square::G1);
        assert_eq!(m.get_to(), Square::C5);

        assert_eq!(m.get_piece(), Piece::Springer);
        assert_eq!(m.get_c1_piece(), Piece::Stradler);
    }
}