use crate::{bishop_logic, knight_logic, pawn_logic, queen_logic, rook_logic,king_logic, Board, Color, PieceType};

pub fn move_piece(board: &mut [[Board;8];8], row: usize, col: usize, whois:usize){
    let color=match whois{
        0 => Color::White,
        1 => Color::Black,
        _ => panic!()
    };
    let Some(piece) = board[row][col].piece else { todo!() };
    if piece.color == color {
        match piece.piece_type {
            PieceType::Pawn => { pawn_logic::move_pawn(board, piece, row, col)}
            PieceType::Knight => {knight_logic::move_knight(board, piece, row, col)}
            PieceType::Bishop => {bishop_logic::move_bishop(board, piece, row, col)}
            PieceType::Rook => {rook_logic::move_rook(board, piece, row, col)}
            PieceType::Queen => {queen_logic::move_queen(board, piece, row, col)}
            PieceType::King => {king_logic::move_king(board, piece, row, col)}
        }


    }
}