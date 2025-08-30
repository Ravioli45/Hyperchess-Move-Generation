use crate::types::{Bitboard, Color, Piece, Square};
use crate::r#move::{Move, MoveList};
use crate::tables::{get_orth_moves, get_diag_moves, get_potential_stradler_captures, get_king_moves, get_death_squares, get_springer_landing_square, get_retractor_lookup, get_springer_captured_square};
//use crate::tables::*;

use std::fmt;
use std::num::ParseIntError;
use std::error;

#[derive(Debug)]
pub struct ReadFenError{}
impl From<ParseIntError> for ReadFenError{
    fn from(_value: ParseIntError) -> Self{
        ReadFenError{}
    }
}
impl fmt::Display for ReadFenError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Problem encountered while reading FEN")
    }
}
impl error::Error for ReadFenError{}

type Result<T> = std::result::Result<T, ReadFenError>;

#[derive(Clone)]
pub struct Position{
    board: [Piece; 64],
    bitboards: [Bitboard; 16],
    zobrist_hash: u64,
    to_play: Color,
    halfmoves: u32,
    fullmoves: u32,
}
impl Position{

    const STARTING_FEN: &'static str = "unbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNU w 0 1";

    fn create_empty() -> Self{
        Position{ 
            board: [Piece::Empty; 64],
            bitboards: [Bitboard::EMPTY; 16],
            zobrist_hash: 0,
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

        //let mut fen_chars = fen.chars();
        let mut fen_parts = fen.split(' ').fuse();

        //[0, 7]
        let mut rank = 7;
        let mut file = 0;

        let mut fen_position = fen_parts.next().ok_or(ReadFenError{})?.chars();
        // read pieces
        while let Some(mut piece) = fen_position.next(){

            if piece.is_ascii_digit(){
                file += piece.to_digit(10).unwrap();
                continue;
            }

            let color = if piece.is_ascii_uppercase() {Color::White} else {Color::Black};

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

            let s = (rank*8 + file) as usize;
            result.board[s] = piece_index;
            result.bitboards[color | piece_index] |= Bitboard(1 << (rank*8 + file));
            result.bitboards[color] |= Bitboard(1 << (rank*8 + file));

            file += 1;

        }

        result.to_play = match fen_parts.next(){
            Some("w") => Color::White,
            Some("b") => Color::Black,
            _ => return Err(ReadFenError{}),
        };
        // skip space after color to move
        //fen_chars.next();

        // half move
        //let halfmoves: String = fen_chars.by_ref().take_while(|c| *c != ' ').collect();

        result.halfmoves = fen_parts.next().ok_or(ReadFenError{})?.parse()?;

        // full move
        let mut fullmoves: &str = fen_parts.next().ok_or(ReadFenError{})?;

        if fullmoves == "-"{
            fullmoves = "0";
        }

        result.fullmoves = fullmoves.parse()?;

        Ok(result)
    }

    /// Place a piece of specified color and type at square, assumes target square is empty
    fn place_piece(&mut self, color: Color, piece: Piece, square: Square){
        assert!(piece != Piece::Empty);
        self.bitboards[color] |= square.into();
        self.bitboards[color | piece] |= square.into();
        self.board[square] = piece;
    }

    /// Place a piece of specified color and type at square, assumes target square has piece
    fn remove_piece(&mut self, color: Color, piece: Piece, square: Square){
        self.bitboards[color] &=  !(Bitboard::from(square));
        self.bitboards[color | piece] &=  !(Bitboard::from(square));
        self.board[square] = Piece::Empty;
    }

    /// Returns a MoveList containing all the psuedolegal moves 
    /// from the curruent position
    pub fn generate_moves(&self) -> MoveList{

        let mut move_list = MoveList::new();

        let not_to_play = !self.to_play;
        let king_square = self.bitboards[self.to_play | Piece::King].bitscanforward_square();
        
        //let immobilized = get_king_moves((self.bitboards[not_to_play | Piece::Immobilizer]).bitscanforward_square());
        let immobilized = if self.bitboards[not_to_play | Piece::Immobilizer].is_empty(){
            Bitboard::EMPTY
        }else{
            get_king_moves((self.bitboards[not_to_play | Piece::Immobilizer]).bitscanforward_square())
        };

        let total_board: Bitboard = self.bitboards[Color::White] | self.bitboards[Color::Black];

        /*
            STRADLER MOVES
        */
        let mut stradlers: Bitboard = self.bitboards[self.to_play | Piece::Stradler] &! immobilized;

        while !stradlers.is_empty(){

            let from: Square = stradlers.pop_lsb_square();
            
            let mut move_bitboard: Bitboard = get_orth_moves(from, total_board) &! total_board;

            while !move_bitboard.is_empty(){

                let to: Square = move_bitboard.pop_lsb_square();

                let maybe_captures: [Bitboard; 4] = 
                    get_potential_stradler_captures(to, self.bitboards[self.to_play | Piece::Stradler] & !Bitboard::from(from));

                let mut m: Move = Move::EMPTY;

                // up
                if !(maybe_captures[0] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c1_piece(self.board[maybe_captures[0].bitscanforward()]);
                }
                // right
                if !(maybe_captures[1] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c2_piece(self.board[maybe_captures[1].bitscanforward()]);
                }
                // down
                if !(maybe_captures[2] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c3_piece(self.board[maybe_captures[2].bitscanforward()]);
                }
                // left
                if !(maybe_captures[3] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c4_piece(self.board[maybe_captures[3].bitscanforward()]);
                }

                let maybe_chameleon_capture: [Bitboard; 4] = 
                    get_potential_stradler_captures(to, self.bitboards[self.to_play | Piece::Chameleon] & !Bitboard::from(from));

                // up
                if !(maybe_chameleon_capture[0] & self.bitboards[not_to_play | Piece::Stradler]).is_empty(){
                    m.set_c1_piece(Piece::Stradler);
                }
                // right
                if !(maybe_chameleon_capture[1] & self.bitboards[not_to_play | Piece::Stradler]).is_empty(){
                    m.set_c2_piece(Piece::Stradler);
                }
                // left
                if !(maybe_chameleon_capture[2] & self.bitboards[not_to_play | Piece::Stradler]).is_empty(){
                    m.set_c3_piece(Piece::Stradler);
                }
                // down
                if !(maybe_chameleon_capture[3] & self.bitboards[not_to_play | Piece::Stradler]).is_empty(){
                    m.set_c4_piece(Piece::Stradler);
                }

                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::Stradler);

                move_list.add_move(m);

            }

        }

        //println!("bg");

        /*
            COORDINATOR MOVES
         */
        let mut coordinators: Bitboard = self.bitboards[self.to_play | Piece::Coordinator] &! immobilized;

        // maximum of one coordinator on the board
        if !coordinators.is_empty(){
            let from = coordinators.pop_lsb_square();

            let mut move_bitboard = (get_orth_moves(from, total_board) | get_diag_moves(from, total_board)) &! total_board;
            
            while !move_bitboard.is_empty(){

                let to = move_bitboard.pop_lsb_square();

                let mut m = Move::EMPTY;

                // coordinator king captures
                
                let coord_kind_death: [Bitboard; 2] = get_death_squares(to, king_square);

                if !(coord_kind_death[0] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c1_piece(self.board[coord_kind_death[0].bitscanforward()]);
                }
                if !(coord_kind_death[1] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c2_piece(self.board[coord_kind_death[1].bitscanforward()]);
                }

                // coordinator chameleon death squares (only captures king)
                let mut coord_chameleon_death: [Bitboard; 4] = [Bitboard::EMPTY; 4];
                let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon];

                let mut i: usize = 0;
                while !chameleons.is_empty(){
                    let c = chameleons.pop_lsb_square();
                    let d = get_death_squares(to, c);
                    coord_chameleon_death[i] = d[0];
                    coord_chameleon_death[i+1] = d[1];
                    i += 2;
                }

                if !(coord_chameleon_death[0] & self.bitboards[not_to_play | Piece::King]).is_empty(){
                    m.set_c5_bit(true);
                }
                if !(coord_chameleon_death[1] & self.bitboards[not_to_play | Piece::King]).is_empty(){
                    m.set_c6_bit(true);
                }
                if !(coord_chameleon_death[2] & self.bitboards[not_to_play | Piece::King]).is_empty(){
                    m.set_c7_bit(true);
                }
                if !(coord_chameleon_death[3] & self.bitboards[not_to_play | Piece::King]).is_empty(){
                    m.set_c8_bit(true);
                }


                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::Coordinator);

                move_list.add_move(m);

            }

        }

        //println!("g");

        /*
            SPRINGER MOVES
         */
        let mut springers = self.bitboards[self.to_play | Piece::Springer] &! immobilized;

        while !springers.is_empty(){
            let from = springers.pop_lsb_square();

            let mut move_bitboard = get_orth_moves(from, total_board) | get_diag_moves(from, total_board);
            let mut maybe_captures = move_bitboard & self.bitboards[not_to_play];
            move_bitboard &= !total_board;

            // just moves
            while !move_bitboard.is_empty(){
                let to = move_bitboard.pop_lsb_square();

                let mut m = Move::EMPTY;

                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::Springer);

                move_list.add_move(m);
            }

            // just captures
            while !maybe_captures.is_empty(){
                let capturing = maybe_captures.pop_lsb_square();
                let landing = get_springer_landing_square(from, capturing);

                if !(landing &! total_board).is_empty(){
                    let mut m = Move::EMPTY;

                    m.set_from(from);
                    m.set_to(landing.bitscanforward_square());
                    m.set_piece(Piece::Springer);
                    m.set_c1_piece(self.board[capturing]);

                    move_list.add_move(m);
                }
            }

        }

        /* 
            chameleon moves (skipping for now)
        */
        let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon] &! immobilized;
        while !chameleons.is_empty(){
            let from: Square = chameleons.pop_lsb_square();

            let mut move_bitboard = get_orth_moves(from, total_board) | get_diag_moves(from, total_board);

            // used for checking potential chamleon springer captures
            let mut maybe_springer_captures = move_bitboard & self.bitboards[not_to_play | Piece::Springer];

            // used for checking when moves may be stradler captures
            let orth_moves: Bitboard = get_orth_moves(from, total_board);

            // potential king and retractor captures
            let king_mask: Bitboard = get_king_moves(from);

            move_bitboard &= !total_board;

            while !move_bitboard.is_empty(){
                let to: Square = move_bitboard.pop_lsb_square();
                let bitboard_to = Bitboard::from(to);

                // potential stradler
                if bitboard_to & orth_moves != Bitboard::EMPTY{

                }

                // potential king and retractor
                if bitboard_to & king_mask != Bitboard::EMPTY{

                }

                // everything else (so just the coordinator i guess???)

                
            }

        }

        /*
            RETRACTOR MOVES
         */
        let mut retractor = self.bitboards[self.to_play | Piece::Retractor] &! immobilized;

        // only one retractor
        if !retractor.is_empty(){

            let from: Square = retractor.pop_lsb_square();

            let mut move_bitboard = (get_orth_moves(from, total_board) | get_diag_moves(from, total_board)) &! total_board;

            // filter out moves that might be captures
            let mut maybe_captures = move_bitboard & get_king_moves(from);

            move_bitboard &= !maybe_captures;

            while !move_bitboard.is_empty(){

                let to: Square = move_bitboard.pop_lsb_square();

                let mut m = Move::EMPTY;

                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::Retractor);

                move_list.add_move(m);
            }

            while !maybe_captures.is_empty(){

                let to: Square = maybe_captures.pop_lsb_square();

                let mut m = Move::EMPTY;

                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::Retractor);

                // captures here
                let capturing = get_retractor_lookup(from, to);
                if !(capturing & self.bitboards[not_to_play]).is_empty(){
                    m.set_c1_piece(self.board[capturing.bitscanforward()]);
                }

                move_list.add_move(m);
            }

        }

        /* 
            IMMBOLIZER MOVES
        */
        // immobilizor can't capture
        let chameleon_immobilzed: Bitboard = {
            let mut chameleons = self.bitboards[not_to_play | Piece::Chameleon];
            let mut result = Bitboard(0);
            while !chameleons.is_empty(){
                result |= get_king_moves(chameleons.pop_lsb_square());
            }
            result
        };

        let mut immobilizor = self.bitboards[self.to_play | Piece::Immobilizer] & !(immobilized | chameleon_immobilzed);
        

        if !immobilizor.is_empty(){

            let from = immobilizor.pop_lsb_square();

            let mut move_bitboard = (get_orth_moves(from, total_board) | get_diag_moves(from, total_board)) &! total_board;

            while !move_bitboard.is_empty(){
                let to = move_bitboard.pop_lsb_square();

                let mut m = Move::EMPTY;

                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::Immobilizer);

                move_list.add_move(m);
            }
        }

        /*
            KING MOVES
         */
        let mut king: Bitboard = self.bitboards[self.to_play | Piece::King] &! immobilized;

        if !king.is_empty(){
            //let coord_square = self.bitboards[self.to_play | Piece::Coordinator].bitscanforward_square();
            let from = king.pop_lsb_square();

            let mut move_bitboard = get_king_moves(from) &! self.bitboards[self.to_play];

            while !move_bitboard.is_empty(){

                let to = move_bitboard.pop_lsb_square();

                let mut m = Move::EMPTY;

                let king_coord_death: [Bitboard; 2] = if self.bitboards[self.to_play | Piece::Coordinator].is_empty(){
                    [Bitboard::EMPTY; 2]
                }
                else{
                    get_death_squares(to, self.bitboards[self.to_play | Piece::Coordinator].bitscanforward_square())
                };

                if !(king_coord_death[0] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c2_piece(self.board[king_coord_death[0].bitscanforward_square()]);
                }
                if !(king_coord_death[1] & self.bitboards[not_to_play]).is_empty(){
                    m.set_c3_piece(self.board[king_coord_death[1].bitscanforward_square()]);
                }

                // king chameleon captures (only captures coordinator)
                let mut king_chameleon_death: [Bitboard; 4] = [Bitboard::EMPTY; 4];
                let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon];

                let mut i: usize = 0;
                while !chameleons.is_empty(){
                    let c = chameleons.pop_lsb_square();
                    let d = get_death_squares(to, c);
                    king_chameleon_death[i] = d[0];
                    king_chameleon_death[i+1] = d[1];
                    i += 2;
                }
                
                if !(king_chameleon_death[0] & self.bitboards[not_to_play | Piece::Coordinator]).is_empty(){
                    m.set_c5_bit(true);
                }
                if !(king_chameleon_death[1] & self.bitboards[not_to_play | Piece::Coordinator]).is_empty(){
                    m.set_c6_bit(true);
                }
                if !(king_chameleon_death[2] & self.bitboards[not_to_play | Piece::Coordinator]).is_empty(){
                    m.set_c7_bit(true);
                }
                if !(king_chameleon_death[3] & self.bitboards[not_to_play | Piece::Coordinator]).is_empty(){
                    m.set_c8_bit(true);
                }


                m.set_from(from);
                m.set_to(to);
                m.set_piece(Piece::King);
                
                // capture by displacement
                m.set_c1_piece(self.board[to]);

                move_list.add_move(m);

            }

        }

        move_list
        /*
        move_list.into_iter().filter(|m| {
            self.make_move(*m);
            let result = !self.is_check();
            self.unmake_move(*m);
            result
        }).collect()
        */
    }

    pub fn make_move(&mut self, m: Move){
        //println!("m: {m:?}");

        let from = m.get_from();
        let to = m.get_to();
        let piece_type = m.get_piece();
        //let piece_type = self.board[from];
        debug_assert!(piece_type == self.board[from]);
        let not_to_play = !self.to_play;
        //let mut captures = m.get_capture_bits();
        //let mut capture_toggle = Bitboard(0);
        let king_square: Square = self.bitboards[self.to_play | Piece::King].bitscanforward_square();

        // for handling captures
        match piece_type{
            Piece::Empty => unreachable!(),
            Piece::Stradler => {
                
                let maybe_captures = 
                    get_potential_stradler_captures(to, Bitboard::UNUSED & !Bitboard::from(from));
                
                // p = type of captures piece (may be Piece::Empty), s = square where piece was captured
                
                // up
                if m.get_c1_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c1_piece(), maybe_captures[0].bitscanforward_square());
                    self.remove_piece(not_to_play, p, s);
                }
                // right
                if m.get_c2_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c2_piece(), maybe_captures[1].bitscanforward_square());
                    self.remove_piece(not_to_play, p, s);
                }
                // down
                if m.get_c3_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c3_piece(), maybe_captures[2].bitscanforward_square());
                    self.remove_piece(not_to_play, p, s);
                }
                // left
                if m.get_c4_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c4_piece(), maybe_captures[3].bitscanforward_square());
                    self.remove_piece(not_to_play, p, s);
                }

            },
            Piece::Coordinator => {

                // coord king captures
                let coord_king_death = get_death_squares(to, king_square);
                //println!("{:?}", coord_king_death);

                if m.get_c1_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c1_piece(), coord_king_death[0].bitscanforward_square());
                    self.remove_piece(not_to_play, p, s);
                }
                if m.get_c2_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c2_piece(), coord_king_death[1].bitscanforward_square());
                    self.remove_piece(not_to_play, p, s);
                }

                // coordinator chameleon death squares (only captures king)
                let mut coord_chameleon_death: [Bitboard; 4] = [Bitboard::EMPTY; 4];
                let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon];

                let mut i: usize = 0;
                while !chameleons.is_empty(){
                    let c = chameleons.pop_lsb_square();
                    let d = get_death_squares(to, c);
                    coord_chameleon_death[i] = d[0];
                    coord_chameleon_death[i+1] = d[1];
                    i += 2;
                }

                if m.get_c5_bit() && !coord_chameleon_death[0].is_empty(){
                    self.remove_piece(not_to_play, Piece::King, coord_chameleon_death[0].bitscanforward_square());
                }
                if m.get_c6_bit() && !coord_chameleon_death[1].is_empty(){
                    self.remove_piece(not_to_play, Piece::King, coord_chameleon_death[1].bitscanforward_square());
                }
                if m.get_c7_bit() && !coord_chameleon_death[2].is_empty(){
                    self.remove_piece(not_to_play, Piece::King, coord_chameleon_death[2].bitscanforward_square());
                }
                if m.get_c8_bit() && !coord_chameleon_death[3].is_empty(){
                    self.remove_piece(not_to_play, Piece::King, coord_chameleon_death[3].bitscanforward_square());
                }

            },
            Piece::Springer => {
                if m.get_c1_piece() != Piece::Empty{
                    let captured_on = get_springer_captured_square(from, to);
                    self.remove_piece(not_to_play, m.get_c1_piece(), captured_on.bitscanforward_square());
                }

            },
            Piece::Chameleon => {
                
            },
            Piece::Retractor => {
                if m.get_c1_piece() != Piece::Empty{
                    let captured_on = get_retractor_lookup(from, to);
                    self.remove_piece(not_to_play, m.get_c1_piece(), captured_on.bitscanforward_square());
                }
            },
            Piece::Immobilizer => {},
            Piece::King => {
                
                if m.get_c1_piece() != Piece::Empty{
                    self.remove_piece(not_to_play, m.get_c1_piece(), to);
                }

                let king_coord_death: [Bitboard; 2] = if self.bitboards[self.to_play | Piece::Coordinator].is_empty(){
                    [Bitboard::EMPTY; 2]
                }
                else{
                    get_death_squares(to, self.bitboards[self.to_play | Piece::Coordinator].bitscanforward_square())
                };

                if m.get_c2_piece() != Piece::Empty{
                    self.remove_piece(not_to_play, m.get_c2_piece(), king_coord_death[0].bitscanforward_square());
                }
                if m.get_c3_piece() != Piece::Empty{
                    self.remove_piece(not_to_play, m.get_c3_piece(), king_coord_death[1].bitscanforward_square());
                }

                // king chameleon captures (only captures coordinator)
                let mut king_chameleon_death: [Bitboard; 4] = [Bitboard::EMPTY; 4];
                let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon];

                let mut i: usize = 0;
                while !chameleons.is_empty(){
                    let c = chameleons.pop_lsb_square();
                    let d = get_death_squares(to, c);
                    king_chameleon_death[i] = d[0];
                    king_chameleon_death[i+1] = d[1];
                    i += 2;
                }

                if m.get_c5_bit(){
                    self.remove_piece(not_to_play, Piece::Coordinator, king_chameleon_death[0].bitscanforward_square());
                }
                if m.get_c6_bit(){
                    self.remove_piece(not_to_play, Piece::Coordinator, king_chameleon_death[1].bitscanforward_square());
                }
                if m.get_c7_bit(){
                    self.remove_piece(not_to_play, Piece::Coordinator, king_chameleon_death[2].bitscanforward_square());
                }
                if m.get_c8_bit(){
                    self.remove_piece(not_to_play, Piece::Coordinator, king_chameleon_death[3].bitscanforward_square());
                }
            },
        }

        // update piece that moved
        let toggle_bitboard = Bitboard(1 << (from as u64) | 1 << (to as u64)); 

        self.bitboards[self.to_play | piece_type] ^= toggle_bitboard;
        self.bitboards[self.to_play] ^= toggle_bitboard;


        // update mailbox
        //self.board[to] = self.board[from];
        self.board[to] = piece_type;
        self.board[from] = Piece::Empty;

        // switch color to play
        self.to_play = !self.to_play;

        self.halfmoves += 1;

        debug_assert!(self.is_consistent(), "{}, {}, {}, {:?}", from, to, piece_type, m);
    }

    pub fn unmake_move(&mut self, m: Move){
        //println!("u: {m:?}");
        //println!("{:?}", self.to_play);

        let from = m.get_from();
        let to = m.get_to();
        let piece_type = m.get_piece();
        //let piece_type = self.board[to];
        debug_assert!(piece_type == self.board[to]);

        // switch color to play
        self.to_play = !self.to_play;
        let not_to_play = !self.to_play;

        // update bitboards
        let toggle_bitboard = Bitboard(1 << (from as u64) | 1 << (to as u64)); 

        self.bitboards[self.to_play | piece_type] ^= toggle_bitboard;
        self.bitboards[self.to_play] ^= toggle_bitboard;
        //println!("{:?}", self.bitboards[self.to_play | Piece::King]);

        // update mailbox
        self.board[from] = self.board[to];
        self.board[to] = Piece::Empty;

        

        self.halfmoves -= 1;

        //let mut captures = m.get_capture_bits();
        let king_square: Square = self.bitboards[self.to_play | Piece::King].bitscanforward_square();

        match piece_type{
            Piece::Empty => unreachable!(),
            Piece::Stradler => {
                //println!("{:?}", self.to_play);
                //println!("{:?}", self.bitboards[not_to_play | Piece::Stradler]);
                let maybe_captures = 
                    get_potential_stradler_captures(to, Bitboard::UNUSED & !Bitboard::from(from));
                
                //println!("{maybe_captures:?}");

                // up
                if m.get_c1_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c1_piece(), maybe_captures[0].bitscanforward_square());
                    self.place_piece(not_to_play, p, s);
                }
                // right
                if m.get_c2_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c2_piece(), maybe_captures[1].bitscanforward_square());
                    self.place_piece(not_to_play, p, s);
                }
                // down
                if m.get_c3_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c3_piece(), maybe_captures[2].bitscanforward_square());
                    self.place_piece(not_to_play, p, s);
                }
                // left
                if m.get_c4_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c4_piece(), maybe_captures[3].bitscanforward_square());
                    self.place_piece(not_to_play, p, s);
                }
            },
            Piece::Coordinator => {

                // coord king captures
                let coord_king_death = get_death_squares(to, king_square);

                if m.get_c1_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c1_piece(), coord_king_death[0].bitscanforward_square());
                    self.place_piece(not_to_play, p, s);
                }
                if m.get_c2_piece() != Piece::Empty{
                    let (p, s): (Piece, Square) = (m.get_c2_piece(), coord_king_death[1].bitscanforward_square());
                    self.place_piece(not_to_play, p, s);
                }

                // coordinator chameleon death squares (only captures king)
                let mut coord_chameleon_death: [Bitboard; 4] = [Bitboard::EMPTY; 4];
                let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon];

                let mut i: usize = 0;
                while !chameleons.is_empty(){
                    //println!("{chameleons:?}");
                    let c = chameleons.pop_lsb_square();
                    let d = get_death_squares(to, c);
                    coord_chameleon_death[i] = d[0];
                    coord_chameleon_death[i+1] = d[1];
                    i += 2;
                }

                if m.get_c5_bit() && !coord_chameleon_death[0].is_empty(){
                    self.place_piece(not_to_play, Piece::King, coord_chameleon_death[0].bitscanforward_square());
                }
                if m.get_c6_bit() && !coord_chameleon_death[1].is_empty(){
                    self.place_piece(not_to_play, Piece::King, coord_chameleon_death[1].bitscanforward_square());
                }
                if m.get_c7_bit() && !coord_chameleon_death[2].is_empty(){
                    self.place_piece(not_to_play, Piece::King, coord_chameleon_death[2].bitscanforward_square());
                }
                if m.get_c8_bit() && !coord_chameleon_death[3].is_empty(){
                    self.place_piece(not_to_play, Piece::King, coord_chameleon_death[3].bitscanforward_square());
                }
            },
            Piece::Springer => {
                
                
                if m.get_c1_piece() != Piece::Empty{
                    let captured_on = get_springer_captured_square(from, to);
                    self.place_piece(not_to_play, m.get_c1_piece(), captured_on.bitscanforward_square());
                }

            },
            Piece::Chameleon => {
                
            },
            Piece::Retractor => {
                if m.get_c1_piece() != Piece::Empty{
                    let captured_on = get_retractor_lookup(from, to);
                    self.place_piece(not_to_play, m.get_c1_piece(), captured_on.bitscanforward_square());
                }
            },
            Piece::Immobilizer => {},
            Piece::King => {
                
                if m.get_c1_piece() != Piece::Empty{
                    self.place_piece(not_to_play, m.get_c1_piece(), to);
                }

                let king_coord_death: [Bitboard; 2] = if self.bitboards[self.to_play | Piece::Coordinator].is_empty(){
                    [Bitboard::EMPTY; 2]
                }
                else{
                    get_death_squares(to, self.bitboards[self.to_play | Piece::Coordinator].bitscanforward_square())
                };

                if m.get_c2_piece() != Piece::Empty{
                    self.place_piece(not_to_play, m.get_c2_piece(), king_coord_death[0].bitscanforward_square());
                }
                if m.get_c3_piece() != Piece::Empty{
                    self.place_piece(not_to_play, m.get_c3_piece(), king_coord_death[1].bitscanforward_square());
                }

                let mut king_chameleon_death: [Bitboard; 4] = [Bitboard::EMPTY; 4];
                let mut chameleons = self.bitboards[self.to_play | Piece::Chameleon];

                let mut i: usize = 0;
                while !chameleons.is_empty(){
                    let c = chameleons.pop_lsb_square();
                    let d = get_death_squares(to, c);
                    king_chameleon_death[i] = d[0];
                    king_chameleon_death[i+1] = d[1];
                    i += 2;
                }

                if m.get_c5_bit(){
                    self.place_piece(not_to_play, Piece::Coordinator, king_chameleon_death[0].bitscanforward_square());
                }
                if m.get_c6_bit(){
                    self.place_piece(not_to_play, Piece::Coordinator, king_chameleon_death[1].bitscanforward_square());
                }
                if m.get_c7_bit(){
                    self.place_piece(not_to_play, Piece::Coordinator, king_chameleon_death[2].bitscanforward_square());
                }
                if m.get_c8_bit(){
                    self.place_piece(not_to_play, Piece::Coordinator, king_chameleon_death[3].bitscanforward_square());
                }
            },
        }

        /*
        // update bitboards
        let toggle_bitboard = Bitboard(1 << (from as u64) | 1 << (to as u64)); 

        self.bitboards[not_to_play | piece_type] ^= toggle_bitboard;
        self.bitboards[not_to_play] ^= toggle_bitboard;

        // update mailbox
        self.board[from] = self.board[to];
        self.board[to] = Piece::Empty;

        // switch color to play
        self.to_play = !self.to_play;

        self.halfmoves -= 1;
        */

        debug_assert!(self.is_consistent(), "{:?}", m);
    }

    /// returns true if self.to_play is currently attacking enemy king
    pub fn is_attacking_king(&mut self) -> bool{

        let opponent = !self.to_play;

        //println!("hh");
        let moves: MoveList = self.generate_moves();
        //println!("hhh");

        let is_check = moves.into_iter().any(|m| {
            self.make_move(m);
            let king_gone = self.bitboards[opponent | Piece::King].is_empty();
            self.unmake_move(m);
            king_gone
        });

        is_check
    }

    pub fn is_move_legal(&mut self, m: Move) -> bool{
        self.make_move(m);

        //println!("g");
        let result = !self.is_attacking_king();
        //println!("hhhh");

        //println!("{self}");
        self.unmake_move(m);
        //println!("hhhhh");

        result
    }

    /// returns true if self.to_play color is currently in checkmate
    pub fn is_checkmate(&mut self) -> bool{

        let current_color: Color = self.to_play;
        let current_moves: MoveList = self.generate_moves();

        current_moves.into_iter().all(|m| {
            self.make_move(m);
            //println!("{self}");

            let responses = self.generate_moves();

            let king_is_attacked = responses.into_iter().any(|r| {
                self.make_move(r);
                //println!("{self}");
                let king_gone = self.bitboards[current_color | Piece::King].is_empty();
                self.unmake_move(r);
                //println!("{self}");

                king_gone
            });

            //println!("{self}");
            self.unmake_move(m);
            //println!("{self}");

            king_is_attacked
        })
    }
    
    /// checks if internal state is consistent
    /// 
    /// internal state is consistent if self.bitboards and self.board agree on current position
    /// 
    /// used for debugging purposes
    fn is_consistent(&self) -> bool{

        let mut board_from_bitboard: [Piece; 64] = [Piece::Empty; 64];

        for c in [Color::White, Color::Black]{

            for p in &(Piece::ALL)[1..]{

                let mut piece_bb = self.bitboards[c | *p];

                while !piece_bb.is_empty(){
                    let square = piece_bb.pop_lsb_square();

                    if board_from_bitboard[square] != Piece::Empty{
                        eprintln!("self.bitboards has overlapping pieces on {square:?}");
                        eprintln!("{:?} and {:?}", *p, board_from_bitboard[square]);
                        return false;
                    }
                    board_from_bitboard[square] = *p;       
                }
            }
        }

        for s in Square::ALL{

            if board_from_bitboard[s] != self.board[s]{
                eprintln!("self.board and self.bitboards disagree on board state at {s}");
                eprintln!("{self}");
                eprintln!("{:?}, {:?}", self.board[s], board_from_bitboard[s]);

                return false;
            }

        }

        true
    }
}
impl fmt::Display for Position{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {


        writeln!(f, "   A   B   C   D   E   F   G   H ")?;
        for rank in (0..8).rev(){

            writeln!(f, " +---+---+---+---+---+---+---+---+")?;
            write!(f, "{}", rank+1)?;

            'file: for file in 0..8{

                write!(f, "| ")?;

                let s = rank*8 + file;

                for i in 0..16{

                    if i == 0 || i == 8{continue;}

                    if (self.bitboards[i].0 >> s) & 1 == 1{
                        write!(f, "{} ", Piece::PIECE_SYMBOLS[i])?;
                        continue 'file;
                    }
                }

                write!(f, "  ")?;

            }

            writeln!(f, "|{}", rank+1)?;
        }

        writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        writeln!(f, "   A   B   C   D   E   F   G   H ")?;

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
            assert_eq!(start_position.bitboards[i], START_POS_BITBOARDS[i]);
        }

    }

    #[test]
    fn bad_fen_test(){

        let position: Result<_> = Position::from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert!(position.is_err());
    }

}
