use std::collections::HashMap;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const DARK_SQS: u64 = 0xAA55AA55AA55AA55;
pub const LIGHT_SQS: u64 = !0xAA55AA55AA55AA55;

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = 0x0202020202020202; // FILE_A << 1
pub const FILE_C: u64 = 0x0404040404040404; // FILE_A << 2
pub const FILE_D: u64 = 0x0808080808080808; // FILE_A << 3
pub const FILE_E: u64 = 0x1010101010101010; // FILE_A << 4
pub const FILE_F: u64 = 0x2020202020202020; // FILE_A << 5
pub const FILE_G: u64 = 0x4040404040404040; // FILE_A << 6
pub const FILE_H: u64 = 0x8080808080808080; // FILE_A << 7

pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = 0x000000000000FF00; // RANK_1 << (8 * 1)
pub const RANK_3: u64 = 0x0000000000FF0000; // RANK_1 << (8 * 2)
pub const RANK_4: u64 = 0x00000000FF000000; // RANK_1 << (8 * 3)
pub const RANK_5: u64 = 0x000000FF00000000; // RANK_1 << (8 * 4)
pub const RANK_6: u64 = 0x0000FF0000000000; // RANK_1 << (8 * 5)
pub const RANK_7: u64 = 0x00FF000000000000; // RANK_1 << (8 * 6)
pub const RANK_8: u64 = 0xFF00000000000000; // RANK_1 << (8 * 7)

pub const QUEEN_SIDE: u64 = FILE_A | FILE_B | FILE_C | FILE_D;
pub const KING_SIDE: u64 = FILE_E | FILE_F | FILE_G | FILE_H;
pub const CENTRE_FILES: u64 = FILE_C | FILE_D | FILE_E | FILE_F;
pub const CENTRE: u64 = (FILE_D | FILE_E) & (RANK_4 | RANK_5);

// pub const _sq_to_int: HashMap<&str, u8> = HashMap::from([
//     ("a8", 56), ("b8", 57), ("c8", 58), ("d8", 59), ("e8", 60), ("f8", 61), ("g8", 62), ("h8", 63),
//     ("a7", 48), ("b7", 49), ("c7", 50), ("d7", 51), ("e7", 52), ("f7", 53), ("g7", 54), ("h7", 55),
//     ("a6", 40), ("b6", 41), ("c6", 42), ("d6", 43), ("e6", 44), ("f6", 45), ("g6", 46), ("h6", 47),
//     ("a5", 32), ("b5", 33), ("c5", 34), ("d5", 35), ("e5", 36), ("f5", 37), ("g5", 38), ("h5", 39),
//     ("a4", 24), ("b4", 25), ("c4", 26), ("d4", 27), ("e4", 28), ("f4", 29), ("g4", 30), ("h4", 31),
//     ("a3", 16), ("b3", 17), ("c3", 18), ("d3", 19), ("e3", 20), ("f3", 21), ("g3", 22), ("h3", 23),
//     ("a2", 08), ("b2", 09), ("c2", 10), ("d2", 11), ("e2", 12), ("f2", 13), ("g2", 14), ("h2", 15),
//     ("a1", 00), ("b1", 01), ("c1", 02), ("d1", 03), ("e1", 04), ("f1", 05), ("g1", 06), ("h1", 07),
// ]);

pub fn squares(n: &str) -> Option<u64> {
    Some(1 << (match n.to_lowercase().as_str() {
        "a8" => 7*8+0, "b8" => 7*8+1, "c8" => 7*8+2, "d8" => 7*8+3, "e8" => 7*8+4, "f8" => 7*8+5, "g8" => 7*8+6, "h8" => 7*8+7,
        "a7" => 6*8+0, "b7" => 6*8+1, "c7" => 6*8+2, "d7" => 6*8+3, "e7" => 6*8+4, "f7" => 6*8+5, "g7" => 6*8+6, "h7" => 6*8+7,
        "a6" => 5*8+0, "b6" => 5*8+1, "c6" => 5*8+2, "d6" => 5*8+3, "e6" => 5*8+4, "f6" => 5*8+5, "g6" => 5*8+6, "h6" => 5*8+7,
        "a5" => 4*8+0, "b5" => 4*8+1, "c5" => 4*8+2, "d5" => 4*8+3, "e5" => 4*8+4, "f5" => 4*8+5, "g5" => 4*8+6, "h5" => 4*8+7,
        "a4" => 3*8+0, "b4" => 3*8+1, "c4" => 3*8+2, "d4" => 3*8+3, "e4" => 3*8+4, "f4" => 3*8+5, "g4" => 3*8+6, "h4" => 3*8+7,
        "a3" => 2*8+0, "b3" => 2*8+1, "c3" => 2*8+2, "d3" => 2*8+3, "e3" => 2*8+4, "f3" => 2*8+5, "g3" => 2*8+6, "h3" => 2*8+7,
        "a2" => 1*8+0, "b2" => 1*8+1, "c2" => 1*8+2, "d2" => 1*8+3, "e2" => 1*8+4, "f2" => 1*8+5, "g2" => 1*8+6, "h2" => 1*8+7,
        "a1" => 0*8+0, "b1" => 0*8+1, "c1" => 0*8+2, "d1" => 0*8+3, "e1" => 0*8+4, "f1" => 0*8+5, "g1" => 0*8+6, "h1" => 0*8+7,
        _ => return None

    }))
}

pub fn knight_moves_bb(sq: u64) -> u64 {
    (sq & !FILE_A) >> 17 |
    (sq & !FILE_H) >> 15 |
    (sq & !(FILE_A | FILE_B)) >> 10 |
    (sq & !(FILE_G | FILE_H)) >> 6  |
    (sq & !(FILE_A | FILE_B)) << 6  |
    (sq & !(FILE_G | FILE_H)) << 10 |
    (sq & !FILE_A) << 15 |
    (sq & !FILE_H) << 17
}

pub fn king_moves_bb(sq: u64) -> u64 {
    sq >> 8 | sq << 8   |
    (sq & !FILE_H) << 1 |
    (sq & !FILE_H) << 9 |
    (sq & !FILE_H) >> 7 |
    (sq & !FILE_A) >> 1 |
    (sq & !FILE_A) >> 9 |
    (sq & !FILE_A) << 7
}

pub fn white_pawn_attacks_bb(sq: u64) -> u64 {
    (sq & !FILE_H) << 9 | (sq & !FILE_A) << 7
}

pub fn black_pawn_attacks_bb(sq: u64) -> u64 {
    (sq & !FILE_H) >> 7 | (sq & !FILE_A) >> 9
}

pub fn bb_to_str(bb: u64) -> String {
    format!("{:0>64}", format!("{bb:b}")) // convert to string add leading zeros if necessary
}

pub fn rank_to_str(r: u8) -> String {
    format!("{:0>8}", format!("{r:b}")) // convert to string add leading zeros if necessary
}

pub fn rev_rank(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;

    b
}

#[test]
fn rev_rank_test() {
    assert_eq!(0b11110000, rev_rank(0b00001111));
    assert_eq!(0b10100100, rev_rank(0b00100101));
    assert_eq!(0b10101010, rev_rank(0b01010101));
    assert_eq!(0b01010101, rev_rank(0b10101010));
}

pub fn pp_bb(bb: u64) {
    
    let ranks = [
        rank_to_str(rev_rank(((bb & RANK_8) >> (64-8*1)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_7) >> (64-8*2)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_6) >> (64-8*3)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_5) >> (64-8*4)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_4) >> (64-8*5)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_3) >> (64-8*6)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_2) >> (64-8*7)) as u8)),
        rank_to_str(rev_rank(((bb & RANK_1) >> (64-8*8)) as u8))
    ];

    let mut output = String::new();
    let mut letters = vec!["7", "6", "5", "4", "3", "2", "1"].into_iter();
    output.push_str("8");

    for r in ranks {
        let spaced_r: String = r.chars().flat_map(|c| {
            Some(' ').into_iter().chain(std::iter::once(if c == '1' {'1'} else {'·'}))
        }).collect();

        output.push_str(&spaced_r);
        if let Some(lt) = letters.next() {
            output.push_str(&format!("{}{}", "\n", lt))
        }
    }

    output.push_str("\n~ A B C D E F G H");

    println!("{output}")

}

// pub fn pretty_print_bb(bb: u64) {
//     let bb_str = bb_to_str(bb);
//
//     println!("{bb_str}");
//
//     let mut asdf = String::new();
//     for (n, c) in bb_str.chars().enumerate() {
//         if n != 0 && n % 8 == 0 {
//             asdf.push_str("\n");
//         }
//         asdf.push(c);
//     }
//
//     let x = asdf.split("\n");
//
//     let mut letters = vec!["7 ", "6 ", "5 ", "4 ", "3 ", "2 ", "1 "].into_iter();
//
//     let mut s = "8 ".to_string();
//
//     for st in x {
//
//         let zxc = st.chars().rev().enumerate()
//             .flat_map(|(i, c)| {
//                 (i != 0).then(|| ' ').into_iter().chain(std::iter::once(c))
//             }).collect::<String>();
//
//
//         s.push_str(&zxc);
//         if let Some(lttr) = letters.next() {
//             s.push_str(&format!("{}{}", "\n", lttr))
//         }
//     }
//
//     s.push_str("\n  A B C D E F G H");
//
//     println!("{s}")
// }
