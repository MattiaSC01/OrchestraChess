use crate::board::Board;
use crate::board::ZobristHashHandler;
use crate::muve::Move;
use crate::utils::PieceType;
use crate::utils::COLOR;
use crate::utils::COLOR::{BLACK, WHITE};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn init_zobrist() -> ZobristHashHandler {
    let mut rng = StdRng::seed_from_u64(11122001_u64);
    let mut table: [[u64; 12]; 64] = [[0; 12]; 64];

    let black_to_move: u64 = rng.gen();

    for i0 in 0..12 {
        for i1 in 0..64 {
            table[i1][i0] = rng.gen();
        }
    }

    ZobristHashHandler {
        table,
        black_to_move,
        hash: 0,
    }
}

fn get_index(color: COLOR, piece: PieceType) -> usize {
    piece_val(piece) + 3 * color_val(color)
}

fn piece_val(p: PieceType) -> usize {
    match p {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Bishop => 2,
        PieceType::Rook => 3,
        PieceType::Queen => 4,
        PieceType::King => 5,
        PieceType::Null => {
            panic!("Mucho poco bueno")
        }
    }
}

fn color_val(color: COLOR) -> usize {
    match color {
        WHITE => 2,
        BLACK => 0,
    }
}

impl Board {
    pub fn init_hash(&mut self) {
        let mut hash: u64 = 0;

        if self.color_to_move == BLACK {
            hash ^= self.zobrist.black_to_move
        }

        for rank in 0..8_u8 {
            for file in 0..8_u8 {
                let square = rank * 8 + file;
                let temp = self.get_piece_on_square(square);
                let piece = temp.0;
                if piece == PieceType::Null {
                    continue;
                }
                let color = temp.1;

                let ind = get_index(color, piece);

                hash ^= self.zobrist.table[square as usize][ind];
            }
        }

        self.zobrist.hash = hash;
    }

    pub(crate) fn update_hash(&mut self, mov: Move) {
        if mov.is_en_passant {
            self.init_hash();
            return;
        }
        if mov.is_castling {
            self.init_hash();
            return;
        }

        let color_that_played_move = self.color_to_move.flip();
        self.zobrist.hash ^= self.zobrist.black_to_move;

        if mov.piece_captured != PieceType::Null {
            let ind_captured = get_index(color_that_played_move.flip(), mov.piece_captured);
            self.zobrist.hash ^= self.zobrist.table[mov.end_square as usize][ind_captured];
        }

        let ind_moved = get_index(color_that_played_move, mov.piece_moved);

        if mov.promotion != PieceType::Null {
            let ind_prom = get_index(color_that_played_move, mov.promotion);
            self.zobrist.hash ^= self.zobrist.table[mov.end_square as usize][ind_prom];
        }

        self.zobrist.hash ^= self.zobrist.table[mov.start_square as usize][ind_moved];
        if mov.promotion == PieceType::Null {
            self.zobrist.hash ^= self.zobrist.table[mov.end_square as usize][ind_moved];
        }
    }
}
