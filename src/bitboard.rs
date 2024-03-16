use std::ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Not};
use std::fmt;

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
    pub fn bitscanforward(&self) -> usize{
        assert!(!self.is_empty());
        Self::MAGIC_TABLE[((self.0 & self.0.wrapping_neg()).wrapping_mul(Self::MAGIC)).wrapping_shr(58) as usize]
    }
    pub fn pop_lsb(&mut self) -> usize{
        assert!(!self.is_empty());
        let lsb = self.bitscanforward();
        self.0 &= self.0-1;
        lsb
    }
}

mod test{

    #[allow(unused)]
    use super::*;

    #[test]
    fn test_bsf(){
        for i in 0..64{
            let bb = Bitboard(1 << i);
            assert!(bb.bitscanforward() == i);
        }
    }

    #[test]
    fn test_poplsb(){
        for i in 0..64{
            for j in i+1..64{
                let mut bb = Bitboard(1 << i | 1 << j);
                assert!(bb.pop_lsb() == i);
                assert!(bb.pop_lsb() == j);
            }
        }
    }
}

