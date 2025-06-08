use crate::{Piece, Square};

use std::fmt;
use std::mem::transmute;

// move info in 32 bits
// 6 bits (0x3f): from
// 6 bits (0xFC0): to
// 3 bits (0x7000): piece type that moved
//
// remaining bits describe captures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(u32);
impl Move{
    pub fn get_from(&self) -> Square{
        //(self.0 & 0x3f) as usize
        unsafe{
            transmute::<usize, Square>((self.0 & 0x3f) as usize)
        }
    }
    pub fn set_from(&mut self, s: Square){
        self.0 |= s as u32;
    }

    pub fn get_to(&self) -> Square{
        //((self.0 & 0xFC0) >> 6) as usize
        unsafe{
            transmute::<usize, Square>(((self.0 & 0xFC0) >> 6) as usize)
        }
    }
    pub fn set_to(&mut self, s: Square){
        self.0 |= (s as u32) << 6;
    }

    pub fn get_piece(&self) -> Piece{
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x7000) >> 12) as usize)
        }
    }
    pub fn set_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 12;
    }

    pub fn get_c1_piece(&self) -> Piece{
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x38000) >> 15) as usize)
        }
    }
    pub fn set_c1_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 15;
    }

    pub fn get_c2_piece(&self) -> Piece{
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x1C0000) >> 18) as usize)
        }
    }
    pub fn set_c2_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 18;
    }

    pub fn get_c3_piece(&self) -> Piece{
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0xE00000) >> 21) as usize)
        }
    }
    pub fn set_c3_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 21;
    }

    pub fn get_c4_piece(&self) -> Piece{
        unsafe{
            transmute::<usize, Piece>(((self.0 & 0x7000000) >> 24) as usize)
        }
    }
    pub fn set_c4_piece(&mut self, p: Piece){
        self.0 |= (p as u32) << 24;
    }

    pub fn get_c5_bit(&self) -> bool{
       ((self.0 & 0x8000000) >> 27) != 0
    }
    pub fn set_c5_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 27;
    }

    pub fn get_c6_bit(&self) -> bool{
       ((self.0 & 0x10000000) >> 28) != 0
    }
    pub fn set_c6_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 28;
    }

    pub fn get_c7_bit(&self) -> bool{
       ((self.0 & 0x20000000) >> 29) != 0
    }
    pub fn set_c7_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 29;
    }

    pub fn get_c8_bit(&self) -> bool{
       ((self.0 & 0x40000000) >> 30) != 0
    }
    pub fn set_c8_bit(&mut self, b: bool){
        self.0 |= (b as u32) << 30;
    }

}
impl fmt::Display for Move{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        writeln!(f, "{}{}", Square::NAMES[self.get_from()], Square::NAMES[self.get_to()])
    }
}


#[cfg(test)]
mod test{

    use crate::{Piece, Square};
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