use crate::engine::position::game_state::GameState;

impl GameState {
/** インデックスから駒の可動範囲を調べる */
    #[inline(always)]
    pub fn get_valid_moves(&self, loc: u8) -> u64 {
        let bit_mask = 1u64 << loc;

        // 駒の種類を格納したボードによる判別。処理が遅かった。
        // if let Some(piece) = self.piecet.0.get(loc as usize) {
        //     // println!("{:?}", piece);
        // }

        // match self.piecet.0[loc as usize] {
        //     Piece::BPawn    =>  self.generate_pawn_moves(loc), 
        //     Piece::WPawn    =>  self.generate_pawn_moves(loc), 
        //     Piece::Knight   =>  self.generate_knight_moves(loc), 
        //     Piece::Bishop   =>  self.generate_bishop_moves(loc), 
        //     Piece::Rook     =>  self.generate_rook_moves(loc), 
        //     Piece::Queen    =>  self.generate_queen_moves(loc), 
        //     Piece::King     =>  self.generate_king_moves(loc), 
        //     _ => { 0x0 }
        // }

        // wpawn bpawnに分けるかもしれない
        if self.pawn & bit_mask != 0 { self.generate_pawn_moves(loc) }
        else if self.knight & bit_mask  != 0 { self.generate_knight_moves(loc)  }
        else if self.bishop & bit_mask  != 0 { self.generate_bishop_moves(loc)  }
        else if self.rook   & bit_mask  != 0 { self.generate_rook_moves(loc)    }
        else if self.queen  & bit_mask  != 0 { self.generate_queen_moves(loc)   }
        else if self.king   & bit_mask  != 0 { self.generate_king_moves(loc)    }
        else { 0x0 }
    }
}
