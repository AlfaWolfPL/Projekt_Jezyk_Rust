use crate::{Board, Piece, Color, INDEX_ROW};
use std::io;

pub(crate) fn move_rook(board: &mut [[Board; 8]; 8], piece: Piece, row: usize, col: usize) {
    println!("Wieża na pozycji {}{}", (b'A' + col as u8) as char, row + 1);
    println!("Podaj index ruchu (np. E5):");

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
        Some(num) if num >= 1 && num <= 8 => (num - 1) as usize,
        _ => {
            println!("Nieprawidłowy wiersz! Użyj 1-8");
            return;
        }
    };

    if is_valid_rook_move(board, &piece, row, col, row_dest, col_dest) {
        // Wykonanie ruchu
        let mut moved_piece = board[row][col].piece.take().unwrap();
        moved_piece.is_moved = true;
        board[row_dest][col_dest].piece = Some(moved_piece);
        board[row][col].piece = None;
        println!("Ruch wieży wykonany!");
    } else {
        println!("Nieprawidłowy ruch wieży!");
    }
}

fn is_valid_rook_move(
    board: &[[Board; 8]; 8],
    piece: &Piece,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> bool {
    // 1. Sprawdź czy ruch jest prostoliniowy (poziomo lub pionowo)
    let same_row = from_row == to_row;
    let same_col = from_col == to_col;

    if !same_row && !same_col {
        return false; // Wieża porusza się tylko w linii prostej
    }

    // 2. Sprawdź czy docelowe pole jest puste lub zawiera pionek przeciwnika
    if let Some(target_piece) = &board[to_row][to_col].piece {
        if target_piece.color == piece.color {
            return false; // Nie można bić własnych pionków
        }
    }

    // 3. Sprawdź czy nie ma przeszkód na drodze
    if same_row {
        // Ruch poziomy
        let col_step = if to_col > from_col { 1 } else { -1 };
        let mut current_col = from_col as isize + col_step;

        while current_col != to_col as isize {
            if board[from_row][current_col as usize].piece.is_some() {
                return false; // Przeszkoda na drodze
            }
            current_col += col_step;
        }
    } else {
        // Ruch pionowy
        let row_step = if to_row > from_row { 1 } else { -1 };
        let mut current_row = from_row as isize + row_step;

        while current_row != to_row as isize {
            if board[current_row as usize][from_col].piece.is_some() {
                return false; // Przeszkoda na drodze
            }
            current_row += row_step;
        }
    }

    true
}