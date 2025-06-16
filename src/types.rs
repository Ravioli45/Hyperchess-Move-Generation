use crate::utils::*;

use std::ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Not};
use std::fmt;
use std::mem::transmute;

/// wrapper struct around u64 to represent
/// bitboard
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

macro_rules! impl_bb_math {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for Bitboard{
            type Output = Self;
            fn $fn(self, other: Self) -> Self::Output{
                Self($trait::$fn(self.0, other.0))
            }
        }
    )*};
}
macro_rules! impl_bb_assign {
    ($($trait:ident, $fn:ident;)*) => {$(
        impl $trait for Bitboard{
            fn $fn(&mut self, other: Self){
                $trait::$fn(&mut self.0, other.0);
            }
        }
    )*};
}
impl_bb_math!{
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}
impl_bb_assign!{
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
}
impl Not for Bitboard{
    type Output = Self;
    fn not(self) -> Self::Output{
        Self(!self.0)
    }
}
impl fmt::Debug for Bitboard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..=7).rev(){
            for file in 0..=7{
                let to_write: char = if ((self.0 >> (rank*8 + file)) & 1) != 0{
                    'X'
                }else{
                    '.'
                };
                write!(f, "{}", to_write)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
impl fmt::LowerHex for Bitboard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)?;
        Ok(())
    }
}
impl Bitboard{
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const UNUSED: Bitboard = Bitboard(u64::MAX);
    const MAGIC: u64 = 0x7ef3ae369961512_u64;
    const MAGIC_TABLE: [usize; 64] = [
        63, 0, 47, 1, 56, 48, 27, 2, 
        60, 57, 49, 41, 37, 28, 16, 3, 
        61, 54, 58, 35, 52, 50, 42, 21, 
        44, 38, 32, 29, 23, 17, 11, 4, 
        62, 46, 55, 26, 59, 40, 36, 15, 
        53, 34, 51, 20, 43, 31, 22, 10, 
        45, 25, 39, 14, 33, 19, 30, 9, 
        24, 13, 18, 8, 12, 7, 6, 5
    ];

    pub fn is_empty(&self) -> bool{
        *self == Self::EMPTY
    }
    pub fn is_unused(&self) -> bool{
        *self == Self::UNUSED
    }

    /// finds index of lsb with debruijn multiplication
    pub const fn bitscanforward(&self) -> usize{
        //assert!(!self.is_empty());
        assert!(self.0 != 0);
        Self::MAGIC_TABLE[((self.0 & self.0.wrapping_neg()).wrapping_mul(Self::MAGIC)).wrapping_shr(58) as usize]
    }
    pub const fn pop_lsb(&mut self) -> usize{
        //assert!(!self.is_empty());
        assert!(self.0 != 0);
        let lsb = self.bitscanforward();
        self.0 &= self.0-1;
        lsb
    }
    pub const fn pop_lsb_square(&mut self) -> Square{
        assert!(self.0 != 0);
        let lsb = self.bitscanforward();
        self.0 &= self.0-1;
        unsafe{
            transmute::<usize, Square>(lsb)
        }
    }
}

num_and_all!{
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum Square{
    A1, B1 ,C1 ,D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6 ,G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}
}
/*
impl fmt::Display for Square{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
*/
impl_indexing!(Square);

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Color{
    White = 0,
    Black = 8,
}
impl From<Color> for usize{
    fn from(value: Color) -> Self {
        value as usize
    }
}
impl<T: Into<usize>> BitOr<T> for Color{
    type Output = usize;

    fn bitor(self, rhs: T) -> Self::Output {
        (self as usize) | (rhs.into())
    }
}
impl Not for Color{
    type Output = Self;

    fn not(self) -> Self::Output{
        match self{
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

num_and_all!{
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Piece{
    Empty = 0,
    Stradler = 1,
    Coordinator = 2,
    Springer = 3,
    Chameleon = 4,
    Retractor = 5,
    Immobilizer = 6,
    King = 7,
}
}
impl Piece{
    pub(crate) const PIECE_SYMBOLS: [char; 16] = ['.', 'P', 'R', 'N', 'B', 'Q', 'U', 'K', '.', 'p', 'r', 'n', 'b', 'q', 'u', 'k'];
}
impl From<Piece> for usize{
    fn from(value: Piece) -> Self{
        value as usize
    }
}
impl<T: Into<usize>> BitOr<T> for Piece{
    type Output = usize;

    fn bitor(self, rhs: T) -> Self::Output {
        (self as usize) | (rhs.into())
    }
}

impl_indexing!(Color, Piece);


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn bitboard_bsf_test(){
        for i in 0..64{
            let bb = Bitboard(1 << i);
            assert!(bb.bitscanforward() == i);
        }
    }

    #[test]
    fn bitboard_pop_lsb_test(){
        for i in 0..64{
            for j in i+1..64{
                let mut bb = Bitboard(1 << i | 1 << j);
                assert!(bb.pop_lsb() == i);
                assert!(bb.pop_lsb() == j);
            }
        }
    }
}