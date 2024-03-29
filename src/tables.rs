#![allow(non_upper_case_globals)]

use crate::{Bitboard, Square};

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

pub const orth_relevant_blockers: [Bitboard; 64] = create_orthogonal_block_masks();
pub const diag_relevant_blockers: [Bitboard; 64] = create_diagonal_block_masks();

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

pub const ORTH_OFFSETS: [usize; 64] = generate_orthogonal_offsets();

pub const DIAG_OFFSETS: [usize; 64] = generate_diagonal_offsets();

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

//pub const DIAG_LOOKUPS: [Bitboard; 5248];

#[cfg(test)]
mod test{

    #![allow(unused)]

    use super::*;

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
            let relevant_blockers = orth_relevant_blockers[square];
            let mut blocker_subset: Bitboard = Bitboard::EMPTY;
            blocker_subset.0 = blocker_subset.0.wrapping_sub(relevant_blockers.0) & relevant_blockers.0;

            while !blocker_subset.is_empty(){
                let index = magic_index(blocker_subset, square_magic, ORTH_THROWAWAY[square]);

                let magic_result = ORTH_LOOKUPS[index + ORTH_OFFSETS[square]];
                assert_eq!(magic_result, generate_orthogonal_moves(square, blocker_subset));

                blocker_subset.0 = blocker_subset.0.wrapping_sub(relevant_blockers.0) & relevant_blockers.0;
            }

        }

    }
}