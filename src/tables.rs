#![allow(non_upper_case_globals)]

use crate::{square, Bitboard, Square};

pub const fn magic_index(blockers: Bitboard, magic: u64, throwaway: u8) -> usize{
    let hash = blockers.0.wrapping_mul(magic);
    let index = (hash >> throwaway) as usize;
    index
}

// only defined so it can be used in 
// initialize num_to_edge
const fn const_min(a: i8, b: i8) -> i8{
    if a > b{
        b
    }
    else{
        a
    }
}

const fn initialize_num_to_edge() -> [[i8; 8]; 64]{
    let mut rank = 0;
    let mut file;
    let mut result = [[0; 8]; 64];

    while rank < 8{
        file = 0;
        while file < 8{
            let n = 7-rank;
            let e = 7-file;
            let s = rank;
            let w = file;
            result[(rank*8 + file) as usize] = [
                n,
                e,
                s,
                w,
                const_min(n, e),
                const_min(s, e),
                const_min(s, w),
                const_min(n, w)
            ];
            file += 1;
        }
        rank+=1;
    }

    result
}

// n, e, s, w, ne, se, sw, nw
const num_squares_to_edge: [[i8; 8]; 64] = initialize_num_to_edge();
const dir_offsets: [i8; 8] = [8, 1, -8, -1, 9, -7, -9, 7];

const fn create_orthogonal_block_masks() -> [Bitboard; 64]{
    let mut result: [Bitboard; 64] = [Bitboard::EMPTY; 64];

    let mut i = 0;
    while i < 64{

        let square_index: usize = Square::ALL[i] as usize;
        let mut blocker_mask = Bitboard::EMPTY;

        let mut j = 0;
        while j < 4{

            let offset = dir_offsets[j];
            let to_edge = num_squares_to_edge[square_index][j];

            let mut k = 1;
            while k < to_edge{

                blocker_mask.0 |= 1 << (square_index as i8 + (offset * k));

                k += 1;
            }

            j += 1;
        }

        result[i] = blocker_mask;
        i += 1;
    }

    result
}

const fn create_diagonal_block_masks() -> [Bitboard; 64]{
    let mut result: [Bitboard; 64] = [Bitboard::EMPTY; 64];

    let mut i = 0;
    while i < 64{

        let square_index: usize = Square::ALL[i] as usize;
        let mut blocker_mask = Bitboard::EMPTY;

        let mut j = 4;
        while j < 8{

            let offset = dir_offsets[j];
            let to_edge = num_squares_to_edge[square_index][j];

            let mut k = 1;
            while k < to_edge{

                blocker_mask.0 |= 1 << (square_index as i8 + (offset * k));

                k += 1;
            }

            j += 1;
        }

        result[i] = blocker_mask;
        i += 1;
    }

    result
}

const fn create_king_masks() -> [Bitboard; 64]{
    let mut result: [Bitboard; 64] = [Bitboard::EMPTY; 64];

    let mut i = 0;
    while i < 64{

        let square_index = Square::ALL[i] as usize;
        let mut blocker_mask = Bitboard::EMPTY;
        
        let mut j = 0;
        while j < 8{

            let offset = dir_offsets[j];
            let to_edge = num_squares_to_edge[square_index][j];
            if to_edge >= 1{
                blocker_mask.0 |= 1 << (square_index as i8 + offset);
            }

            j += 1;
        }
        
        result[i] = blocker_mask;
        i += 1;
    }

    result
}

const fn create_buddy_masks() -> [Bitboard; 64]{
    let mut result: [Bitboard; 64] = [Bitboard::EMPTY; 64];

    let mut i = 0;
    while i < 64{

        let square_index = Square::ALL[i] as usize;
        let mut buddy_mask = Bitboard::EMPTY;

        let mut j = 0;
        while j < 4{
            let offset = dir_offsets[j];
            let to_edge = num_squares_to_edge[square_index][j];

            if to_edge >= 2{
                buddy_mask.0 |= 1 << (square_index as i8 + 2*offset);
            }

            j += 1;
        }

        result[i] = buddy_mask;

        i += 1;
    }

    result
}

pub const fn generate_orthogonal_moves(start: Square, blockers: Bitboard) -> Bitboard{

    let start_index = start as usize;
    let mut result: Bitboard = Bitboard::EMPTY;

    let mut i = 0;
    while i < 4{

        let offset = dir_offsets[i];
        let to_edge = num_squares_to_edge[start_index][i];

        let mut j = 1;
        while j <= to_edge{
            let index = start_index as i8 + offset*j;
            if (blockers.0 >> index)&1 == 1{
                break;
            }
            else{
                result.0 |= 1 << index;
            }
            j += 1;
        }

        i += 1;
    }

    result
}

pub const fn generate_diagonal_moves(start: Square, blockers: Bitboard) -> Bitboard{

    let start_index = start as usize;
    let mut result: Bitboard = Bitboard::EMPTY;

    let mut i = 4;
    while i < 8{

        let offset = dir_offsets[i];
        let to_edge = num_squares_to_edge[start_index][i];

        let mut j = 1;
        while j <= to_edge{
            let index = start_index as i8 + offset*j;
            if (blockers.0 >> index)&1 == 1{
                break;
            }
            else{
                result.0 |= 1 << index;
            }
            j += 1;
        }

        i += 1;
    }

    result
}

pub const fn generate_stradler_captures(start: Square, buddies: Bitboard) -> Bitboard{

    let start_index = start as usize;
    let mut result: Bitboard = Bitboard::EMPTY;

    let mut i = 0;
    while i < 4{
        let offset = dir_offsets[i];
        let to_edge = num_squares_to_edge[start_index][i];

        if to_edge >= 2 && ((buddies.0 >> (start_index as i8 + 2*offset))&1) == 1{
            result.0 |= 1 << (start_index as i8 + offset);
        }

        i += 1;
    }

    result
}

const fn generate_orthogonal_offsets() -> [usize; 64]{
    let mut result: [usize; 64] = [0; 64];
    let mut offset_count: usize = 0;
    let mut i = 0;
    while i < 64{
        result[i] = offset_count;
        offset_count += 1 << (64-ORTH_THROWAWAY[i]);
        i += 1;
    }
    result
}

const fn generate_diagonal_offsets() -> [usize; 64]{
    let mut result: [usize; 64] = [0; 64];
    let mut offset_count: usize = 0;
    let mut i = 0;
    while i < 64{
        result[i] = offset_count;
        offset_count += 1 << (64-DIAG_THROWAWAY[i]);
        i += 1;
    }
    result
}

const fn generate_buddy_offsets() -> [usize; 64]{
    let mut result: [usize; 64] = [0; 64];
    let mut offset_count: usize = 0;
    let mut i = 0;
    while i < 64{
        result[i] = offset_count;
        offset_count += 1 << (64-BUDDY_THROWAWAY[i]);
        i += 1;
    }
    result
}

pub const orth_relevant_blockers: [Bitboard; 64] = create_orthogonal_block_masks();
pub const diag_relevant_blockers: [Bitboard; 64] = create_diagonal_block_masks();
pub const relevant_buddies: [Bitboard; 64] = create_buddy_masks();

pub const KING_MOVE_MASK: [Bitboard; 64] = create_king_masks();

pub const ORTH_MAGICS: [u64; 64] = [
    0x580002011804000_u64, 0x2840004410002008_u64, 0x2100104100200109_u64, 0x80080180841000_u64, 
    0x200041002000820_u64, 0x200840010120028_u64, 0x680020007000180_u64, 0x8600005103802604_u64, 
    0x40800040003189_u64, 0x8400400050082000_u64, 0x2000801000822005_u64, 0x1001000821001004_u64, 
    0x822002090048a00_u64, 0x1801200800c01_u64, 0xa0b000100644200_u64, 0x8080801100004480_u64, 
    0x1880004000200440_u64, 0x810004004200440_u64, 0x12020041801023_u64, 0x3120028402200_u64, 
    0x808004004004200_u64, 0x14008002008004_u64, 0x402040010180201_u64, 0xc2010a0002428401_u64, 
    0x1c00280012181_u64, 0x810004840002000_u64, 0x1200880100080_u64, 0x101480480100081_u64, 
    0x1041480100111500_u64, 0xa000040080020080_u64, 0x284181400125045_u64, 0x3009285200008421_u64, 
    0x8002204001800489_u64, 0x4200184804000_u64, 0x202410251002000_u64, 0xa784811000800800_u64, 
    0x801c01800800_u64, 0x2500800400800200_u64, 0xc010085074000302_u64, 0x50005082002104_u64, 
    0x402400080028020_u64, 0x1004c020014002_u64, 0x200041050010_u64, 0x610008100080800_u64, 
    0x403c008040080800_u64, 0x1800402010080104_u64, 0x1000812862440010_u64, 0x100058059120014_u64, 
    0x2480014004208480_u64, 0x80a01080400580_u64, 0xa04100102000c100_u64, 0x3880812802100280_u64, 
    0x28000900100500_u64, 0x285800400020180_u64, 0x42194a08104400_u64, 0x13000040822100_u64, 
    0x80110042008422_u64, 0x89201a1008442_u64, 0x8820040101822_u64, 0x800082420300101_u64, 
    0x2052001810200402_u64, 0x4100080a8c0001_u64, 0x800208048a01100c_u64, 0x240c80410a_u64
];
pub const DIAG_MAGICS: [u64; 64] = [
    0x8400414004202a0_u64, 0x8a52540c04034010_u64, 0x41550501000005_u64, 0x180404208814019c_u64, 
    0x400c042008800100_u64, 0x8012028aa0001000_u64, 0x1001465010080108_u64, 0x8000a20810881801_u64, 
    0x4005010a0200_u64, 0x100c60a04141088_u64, 0x46088802c48200_u64, 0x8400140400980002_u64, 
    0x420210240200_u64, 0x1051008050a00_u64, 0x40e8020101194001_u64, 0x10104108281224_u64, 
    0x42002008010100_u64, 0x8008000312180a00_u64, 0x2008004442040110_u64, 0x102024402120004_u64, 
    0x24000880a00000_u64, 0x211002170023010_u64, 0x800044c882101000_u64, 0xc000800044042101_u64, 
    0x85a00224081014_u64, 0x2a207101010020a_u64, 0x84021204080012_u64, 0x920080001004008_u64, 
    0x200a00a008040_u64, 0x8852081026080202_u64, 0x204038111081100_u64, 0x840100c081044800_u64, 
    0x2048180408412402_u64, 0x8004040400210102_u64, 0x810802080040800_u64, 0x8000208020080201_u64, 
    0x40024010050100_u64, 0x20018100008044_u64, 0x18410408850480_u64, 0x8090a08500018c02_u64, 
    0x4004010840350800_u64, 0x204010c02001044_u64, 0x441040205044200_u64, 0x1220202001424_u64, 
    0x1080102400408_u64, 0xc04a108102000300_u64, 0x8030404000880_u64, 0xc06040400624480_u64, 
    0x74020104204800_u64, 0x8041c824100000_u64, 0x1a00108048180048_u64, 0x1880042184044000_u64, 
    0x28480c009024210_u64, 0x1400100210070020_u64, 0x40208421420c0004_u64, 0xa2a0080901c88480_u64, 
    0x460840190900880_u64, 0x40011401010829_u64, 0x310450884c81800_u64, 0x4010000002050400_u64, 
    0x2000802028902401_u64, 0x2020000420440110_u64, 0x4c1020600a208314_u64, 0x9104205449020010_u64,
];
pub const STRADLER_MAGICS: [u64; 64] = [
    0x6000680210a00008_u64, 0xb1a41022028404c_u64, 0x4200200840000800_u64, 0x5200102000029422_u64, 
    0x228004c108001002_u64, 0xd00028a04000000_u64, 0x1840410402004010_u64, 0x480108102000101_u64, 
    0x8010018028000c00_u64, 0x1400a020020001_u64, 0xc484003862080002_u64, 0x521805011105020_u64, 
    0x1010a10820001002_u64, 0x201084612850080c_u64, 0x2aa083900500004_u64, 0x420c0109010068_u64, 
    0x201090238801400b_u64, 0x10080840c220060a_u64, 0x2020210011260044_u64, 0x1020120402044902_u64, 
    0x410221024200000c_u64, 0x20016400400007c_u64, 0x818005040a000020_u64, 0x3840240000832111_u64, 
    0x1848c3080360444_u64, 0xa20001004128020_u64, 0x4208529800500000_u64, 0x800490420a420041_u64, 
    0x800400a280090000_u64, 0x8004090420120401_u64, 0x1010a20404020089_u64, 0xc00808900010000_u64, 
    0x20602020024180_u64, 0x101110011184a500_u64, 0x6230442010410_u64, 0x2001100090900884_u64, 
    0xa20812040d08_u64, 0x82140104200214_u64, 0x100146002d000080_u64, 0x5184064a02001502_u64, 
    0x80808200080042_u64, 0x10704880042124_u64, 0x182000d28410_u64, 0x910018410208011_u64, 
    0x1000188400208805_u64, 0x51000a080090101_u64, 0x9001080080049_u64, 0x810910084112801_u64, 
    0x20800100c0002000_u64, 0x4000102040000800_u64, 0x80080008a0004200_u64, 0x920016005401640c_u64, 
    0xc80010003002110_u64, 0x10001020a021044_u64, 0x400210002000418_u64, 0xa40804050110071c_u64, 
    0x2044008200820010_u64, 0x200300000400298_u64, 0x900840288100438_u64, 0x400400000888a014_u64, 
    0x120400500a0a0032_u64, 0x40000000024015_u64, 0x502d001620460404_u64, 0x2a2000000430002_u64
];

pub const RETRACTOR_MAGICS: [u64; 64] = [
    0x8100048804400_u64, 0xa004012501082080_u64, 0x100000a40001c200_u64, 0x11002c4010200000_u64, 
    0x90001048488400c_u64, 0x50c00000180040a0_u64, 0x100008414021005_u64, 0x8895200400060020_u64,
    0x4800410004000410_u64, 0x44026000c3804002_u64, 0x9000c20200000100_u64, 0x180061000400001c_u64,
    0xac004a8010020000_u64, 0xa00184081221140_u64, 0x4100821010084004_u64, 0x81000ac140014900_u64,
    0xa004001080012_u64, 0x2040260000bc308_u64, 0x230012201800000_u64, 0x88041082080505_u64,
    0x1011804c00200400_u64, 0x72002440002202_u64, 0x1009220010801_u64, 0x4043440210000120_u64,
    0x1208444010808200_u64, 0x4000a00408840080_u64, 0x100204420010_u64, 0x848080061009000_u64,
    0x400911008080c0_u64, 0x2004026001008_u64, 0x143891a013000854_u64, 0x24430a200424400_u64,
    0x44200808488044_u64, 0x3404c80402600201_u64, 0x80002290012e0010_u64, 0x2820222800910401_u64,
    0x8010d0400308060_u64, 0x20040200c08801_u64, 0x1000080100821018_u64, 0x20000484002200_u64,
    0x45101080000800_u64, 0x4010040820040884_u64, 0x200000201000c200_u64, 0x4800281008006100_u64,
    0x80000294020840_u64, 0x200002002440_u64, 0x18004011000c20_u64, 0x4000120100800822_u64,
    0x10000c0000800190_u64, 0x40000a0020200244_u64, 0xa01888010100823_u64, 0x80061_u64,
    0x200005008a06804c_u64, 0x1002004020254026_u64, 0x5802aa013_u64, 0x82000001808010c3_u64,
    0x204009108010_u64, 0x80a40800000a4408_u64, 0x600002288048_u64, 0xd020000802100088_u64,
    0xc04d0082000000c_u64, 0x4c01000000000011_u64, 0x80004040009d1001_u64, 0x50300000862000_u64
];

pub const ORTH_THROWAWAY: [u8; 64] = {
    let mut result: [u8; 64] = [0; 64];
    let mut i = 0;
    while i < 64{
        result[i] = orth_relevant_blockers[i].0.count_zeros() as u8;
        i += 1;
    }
    result
};

pub const DIAG_THROWAWAY: [u8; 64] = {
    let mut result: [u8; 64] = [0; 64];
    let mut i = 0;
    while i < 64{
        result[i] = diag_relevant_blockers[i].0.count_zeros() as u8;
        i += 1;
    }
    result
};

pub const BUDDY_THROWAWAY: [u8; 64] = {
    let mut result: [u8; 64] = [0; 64];
    let mut i = 0;
    while i < 64{
        result[i] = relevant_buddies[i].0.count_zeros() as u8;
        i += 1;
    }
    result
};

pub const ORTH_OFFSETS: [usize; 64] = generate_orthogonal_offsets();

pub const DIAG_OFFSETS: [usize; 64] = generate_diagonal_offsets();

pub const STRADLER_OFFSETS: [usize; 64] = generate_buddy_offsets();

pub const ORTH_LOOKUPS: [Bitboard; 102400] = {
    let mut result: [Bitboard; 102400] = [Bitboard::UNUSED; 102400];

    let mut i = 0;
    while i < 64{

        let square_magic: u64 = ORTH_MAGICS[i];
        let relevant_blockers: Bitboard = orth_relevant_blockers[i];
        let mut blocker_subset: Bitboard = Bitboard::EMPTY;

        let mut j = 0;
        while j < (1 << (64-ORTH_THROWAWAY[i])){
            blocker_subset.0 = blocker_subset.0.wrapping_sub(relevant_blockers.0) & relevant_blockers.0;

            let index = magic_index(blocker_subset, square_magic, ORTH_THROWAWAY[i]);

            result[index + ORTH_OFFSETS[i]] = generate_orthogonal_moves(Square::ALL[i], blocker_subset);

            j += 1;
        }

        i += 1;
    }

    result
};

pub const DIAG_LOOKUPS: [Bitboard; 5248] = {
    let mut result: [Bitboard; 5248] = [Bitboard::UNUSED; 5248];

    let mut i = 0;
    while i < 64{

        let square_magic: u64 = DIAG_MAGICS[i];
        let relevant_blockers: Bitboard = diag_relevant_blockers[i];
        let mut blocker_subset: Bitboard = Bitboard::EMPTY;

        let mut j = 0;
        while j < (1 << (64-DIAG_THROWAWAY[i])){
            blocker_subset.0 = blocker_subset.0.wrapping_sub(relevant_blockers.0) & relevant_blockers.0;

            let index = magic_index(blocker_subset, square_magic, DIAG_THROWAWAY[i]);

            result[index + DIAG_OFFSETS[i]] = generate_diagonal_moves(Square::ALL[i], blocker_subset);

            j += 1;
        }

        i += 1;
    }

    result
};

pub const STRADLER_LOOKUPS: [Bitboard; 576] = {
    let mut result: [Bitboard; 576] = [Bitboard::UNUSED; 576];

    let mut i = 0;
    while i < 64{

        let square_magic: u64 = STRADLER_MAGICS[i];
        let relevant_blockers: Bitboard = relevant_buddies[i];
        let mut blocker_subset: Bitboard = Bitboard::EMPTY;

        let mut j = 0;
        while j < (1 << (64-BUDDY_THROWAWAY[i])){
            blocker_subset.0 = blocker_subset.0.wrapping_sub(relevant_blockers.0) & relevant_blockers.0;

            let index = magic_index(blocker_subset, square_magic, BUDDY_THROWAWAY[i]);

            result[index + STRADLER_OFFSETS[i]] = generate_stradler_captures(Square::ALL[i], blocker_subset);

            j += 1;
        }

        i += 1;
    }

    result
};

#[cfg(test)]
mod test{

    #![allow(unused)]

    use std::f64::consts::SQRT_2;

    use super::*;

    const potential_orth_blockers: [Bitboard; 64] = {
        let mut result: [Bitboard; 64] = [Bitboard::EMPTY; 64];

        let mut i = 0;
        while i < 64{
            result[i] = generate_orthogonal_moves(Square::ALL[i], Bitboard::EMPTY);
            i += 1;
        }
        result
    };

    const potential_diag_blockers: [Bitboard; 64] = {
        let mut result: [Bitboard; 64] = [Bitboard::EMPTY; 64];

        let mut i = 0;
        while i < 64{
            result[i] = generate_diagonal_moves(Square::ALL[i], Bitboard::EMPTY);
            i += 1;
        }
        result
    };

    #[test]
    fn throwaway_count_test(){
        for i in 0..64{
            assert_eq!(ORTH_THROWAWAY[i] + orth_relevant_blockers[i].0.count_ones() as u8, 64);
            assert_eq!(DIAG_THROWAWAY[i] + diag_relevant_blockers[i].0.count_ones() as u8, 64);
        }
    }

    #[test]
    fn orth_lookup_test(){
        for square in Square::ALL{

            let square_magic: u64 = ORTH_MAGICS[square];
            let max_blockers = potential_orth_blockers[square];
            //let relevant_blockers = the_board & orth_relevant_blockers[square];

            // blocker_subset represents the board
            let mut blocker_subset: Bitboard = Bitboard::EMPTY;
            blocker_subset.0 = blocker_subset.0.wrapping_sub(max_blockers.0) & max_blockers.0;

            while !blocker_subset.is_empty(){
                let relevant_blockers = blocker_subset & orth_relevant_blockers[square];
                let index = magic_index(relevant_blockers, square_magic, ORTH_THROWAWAY[square]);

                let magic_result = ORTH_LOOKUPS[index + ORTH_OFFSETS[square]];
                //println!("{blocker_subset:?}");
                //println!("{magic_result:?}");
                assert_eq!(magic_result &! blocker_subset, generate_orthogonal_moves(square, blocker_subset));

                blocker_subset.0 = blocker_subset.0.wrapping_sub(max_blockers.0) & max_blockers.0;
            }
        }
    }

    #[test]
    fn diag_lookup_test(){
        for square in Square::ALL{
            let square_magic = DIAG_MAGICS[square];
            //let relevant_blockers = diag_relevant_blockers[square];
            let max_blockers = potential_diag_blockers[square];
            let mut blocker_subset = Bitboard::EMPTY;
            
            blocker_subset.0 = blocker_subset.0.wrapping_sub(max_blockers.0) & max_blockers.0;

            while !blocker_subset.is_empty(){
                let relevant_blockers = blocker_subset & diag_relevant_blockers[square];
                let index = magic_index(relevant_blockers, square_magic, DIAG_THROWAWAY[square]);
                let magic_result = DIAG_LOOKUPS[index + DIAG_OFFSETS[square]];

                assert_eq!(magic_result &! blocker_subset, generate_diagonal_moves(square, blocker_subset));

                blocker_subset.0 = blocker_subset.0.wrapping_sub(max_blockers.0) & max_blockers.0;
            }
        }
    }

    #[test]
    fn stradler_capture_test(){
        for square in Square::ALL{
            let square_magic = STRADLER_MAGICS[square];
            let potential_buddies = relevant_buddies[square];
            let mut buddy_subset = Bitboard::EMPTY;

            buddy_subset.0 = buddy_subset.0.wrapping_sub(potential_buddies.0) & potential_buddies.0;

            while !buddy_subset.is_empty(){
                let index = magic_index(buddy_subset, square_magic, BUDDY_THROWAWAY[square]);
                let magic_result = STRADLER_LOOKUPS[index + STRADLER_OFFSETS[square]];

                assert_eq!(magic_result, generate_stradler_captures(square, buddy_subset));

                buddy_subset.0 = buddy_subset.0.wrapping_sub(potential_buddies.0) & potential_buddies.0;
            }
        }
    }
}