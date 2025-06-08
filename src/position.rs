use crate::utils::{num_and_all, impl_indexing};
use crate::Bitboard;

use std::ops::{Index, IndexMut, BitOr};
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ReadFenError{}

impl From<ParseIntError> for ReadFenError{
    fn from(_value: ParseIntError) -> Self {
        ReadFenError{}
    }
}

type Result<T> = std::result::Result<T, ReadFenError>;

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

num_and_all!{
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Piece{
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
    const PIECE_SYMBOLS: [char; 16] = ['.', 'P', 'R', 'N', 'B', 'Q', 'U', 'K', '.', 'p', 'r', 'n', 'b', 'q', 'u', 'k'];
}
impl From<Piece> for usize{
    fn from(value: Piece) -> Self{
        value as usize
    }
}
impl<T: Into<usize>> BitOr<T> for Piece{
    type Output = usize;

    fn bitor(self, rhs: T) -> Self::Output {
        (self as usize) | (rhs.into() as usize)
    }
}

impl_indexing!(Color, Piece);

pub struct Position{
    boards: [Bitboard; 16],
    to_play: Color,
    halfmoves: u32,
    fullmoves: u32,
}
impl Position{

    const STARTING_FEN: &'static str = "unbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNU w 0 1";

    fn create_empty() -> Self{
        Position{ 
            boards: [Bitboard::EMPTY; 16],
            to_play: Color::White, 
            halfmoves: 0,
            fullmoves: 0,
        }
    }

    pub fn from_start_position() -> Self{
        Self::from_FEN(Self::STARTING_FEN).unwrap()
    }

    #[allow(non_snake_case)]
    pub fn from_FEN(fen: &str) -> Result<Self>{

        let mut result = Self::create_empty();

        let mut fen_chars = fen.chars();

        //[0, 7]
        let mut rank = 7;
        let mut file = 0;

        // read pieces
        while let Some(mut piece) = fen_chars.next(){

            if piece.is_ascii_digit(){
                file += piece.to_digit(10).unwrap();
                continue;
            }

            let color = if piece.is_ascii_uppercase() {Color::White} else {Color::Black};

            // make_ascii_lowercase
            //piece = piece.to_ascii_lowercase();
            piece.make_ascii_lowercase();

            let piece_index = match piece{
                'p' => Piece::Stradler,
                'r' => Piece::Coordinator,
                'n' => Piece::Springer,
                'b' => Piece::Chameleon,
                'q' => Piece::Retractor,
                'u' => Piece::Immobilizer,
                'k' => Piece::King,
                '/' => {
                    rank -= 1;
                    file = 0;
                    continue;
                }
                ' ' => break,
                _ => return Err(ReadFenError{})
            };

            result.boards[color | piece_index] |= Bitboard(1 << (rank*8 + file));
            result.boards[color] |= Bitboard(1 << (rank*8 + file));

            file += 1;

        }

        result.to_play = match fen_chars.next(){
            Some('w') => Color::White,
            Some('b') => Color::Black,
            _ => return Err(ReadFenError{}),
        };
        // skip space after color to move
        fen_chars.next();

        // half move
        let halfmoves: String = fen_chars.by_ref().take_while(|c| *c != ' ').collect();

        result.halfmoves = halfmoves.parse()?;

        // full move
        let fullmoves: String = fen_chars.by_ref().take_while(|c| *c != ' ').collect();

        result.fullmoves = fullmoves.parse()?;

        Ok(result)
    }

}
impl fmt::Display for Position{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {


        writeln!(f, "  A B C D E F G H ")?;
        for rank in (0..8).rev(){

            writeln!(f, " -----------------")?;
            write!(f, "{}", rank+1)?;

            'file: for file in 0..8{

                write!(f, "|")?;

                let s = rank*8 + file;

                for i in 0..16{

                    if i == 0 || i == 8{continue;}

                    if (self.boards[i].0 >> s) & 1 == 1{
                        write!(f, "{}", Piece::PIECE_SYMBOLS[i])?;
                        continue 'file;
                    }
                }

                write!(f, ".")?;

            }

            writeln!(f, "|{}", rank+1)?;
        }

        writeln!(f, " -----------------")?;
        writeln!(f, "  A B C D E F G H ")?;

        Ok(())
    }
}

#[cfg(test)]
mod test{

    use super::*;

    
    const START_POS_BITBOARDS: [Bitboard; 16] = [
        Bitboard(0xFFFF), Bitboard(0xFF00), Bitboard(0x1), Bitboard(0x42), 
        Bitboard(0x24), Bitboard(0x8), Bitboard(0x80), Bitboard(0x10),
        Bitboard(0xffff000000000000), Bitboard(0xff000000000000), Bitboard(0x8000000000000000), Bitboard(0x4200000000000000), 
        Bitboard(0x2400000000000000), Bitboard(0x800000000000000), Bitboard(0x100000000000000), Bitboard(0x1000000000000000),
    ];
    

    #[test]
    fn start_position_test(){

        let start_position = Position::from_start_position();

        assert_eq!(start_position.to_play, Color::White);
        assert_eq!(start_position.halfmoves, 0);
        assert_eq!(start_position.fullmoves, 1);

        for i in 0..16{
            assert_eq!(start_position.boards[i], START_POS_BITBOARDS[i]);
        }

    }

    #[test]
    fn bad_fen_test(){

        let position = Position::from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert!(position.is_err());
    }

}
