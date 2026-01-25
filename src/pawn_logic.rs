use std::io;
use crate::{Board, Color, Piece, INDEX_ROW};

pub(crate) fn move_pawn(board: &mut [[Board; 8]; 8], piece: Piece, row: usize, col: usize){
    if piece.is_moved {
        println!("Ten pionek był już ruszany.");
    } else {
        println!("Ten pionek NIE był jeszcze ruszany (może ruszyć się o 2 pola).");
    }
    println!("Podaj index ruchu (np. A3):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu");

    let input = input.trim().to_uppercase();
    let mut chars = input.chars();
    let col_char = chars.next().unwrap_or(' ');
    let row_char = chars.next().unwrap_or('0');
    let mut binding = [0; 4];
    let col_str = col_char.encode_utf8(&mut binding);
    let col_dest = *INDEX_ROW.get(col_str).expect("Nieprawidłowa kolumna! Użyj A-H");

    let row_dest = match row_char.to_digit(10) {
        Some(num) if (1..=8).contains(&num) => (num - 1) as usize,
        _ => {
            println!("Nieprawidłowy wiersz! Użyj 1-8");
            return;
        }
    };
    if is_valid_pawn_move(board, &piece, row, col, row_dest, col_dest) {
        // Wykonanie ruchu
        let mut moved_piece = board[row][col].piece.take().unwrap();
        moved_piece.is_moved = true;
        board[row_dest][col_dest].piece = Some(moved_piece);
        board[row][col].piece = None;
        println!("Ruch wykonany!");
    } else {
        println!("Nieprawidłowy ruch!");
    }
}

fn is_valid_pawn_move(
    board: &[[Board; 8]; 8],
    piece: &Piece,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> bool {
    let direction = match piece.color {
        Color::White => 1,  // Białe ruszają się "w górę" (zwiększanie wiersza)
        Color::Black => -1, // Czarne ruszają się "w dół" (zmniejszanie wiersza)
    };

    // Konwersja na signed dla łatwiejszych obliczeń
    let from_row_signed = from_row as isize;
    let from_col_signed = from_col as isize;
    let to_row_signed = to_row as isize;
    let to_col_signed = to_col as isize;

    // 1. Ruch prosty o 1 pole
    if from_col == to_col && to_row_signed == from_row_signed + direction {
        return board[to_row][to_col].piece.is_none();
    }

    // 2. Ruch prosty o 2 pola (tylko jeśli nie był ruszany)
    if !piece.is_moved
        && from_col == to_col
        && to_row_signed == from_row_signed + 2 * direction
        && board[to_row][to_col].piece.is_none()
    {
        // Sprawdzenie czy pole pośrednie jest puste
        let intermediate_row = (from_row_signed + direction) as usize;
        return board[intermediate_row][to_col].piece.is_none();
    }

    // 3. Bicie na ukos
    if (to_col_signed == from_col_signed - 1 || to_col_signed == from_col_signed + 1)
        && to_row_signed == from_row_signed + direction
    {
        // Sprawdzenie czy na docelowym polu jest pionek przeciwnika
        if let Some(target_piece) = &board[to_row][to_col].piece {
            return target_piece.color != piece.color;
        }
    }

    false
}