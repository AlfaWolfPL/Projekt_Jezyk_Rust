use crate::{Board, Piece, Color, INDEX_ROW};
use std::io;

pub(crate) fn move_queen(
    board: &mut [[Board; 8]; 8],
    piece: Piece,
    row: usize,
    col: usize
) {
    // Możesz dodać printy tutaj, jeśli chcesz ładne komunikaty.

    // Pobranie ruchu od użytkownika
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu");

    let input = input.trim().to_uppercase();
    let mut chars = input.chars();
    let col_char = chars.next().unwrap_or(' ');
    let row_char = chars.next().unwrap_or('0');

    let mut binding = [0; 4];
    let col_str = col_char.encode_utf8(&mut binding);
    let col_dest = *INDEX_ROW
        .get(col_str)
        .expect("Nieprawidłowa kolumna! Użyj A-H");

    let row_dest = match row_char.to_digit(10) {
        Some(num) if num >= 1 && num <= 8 => (num - 1) as usize,
        _ => {
            // Możesz wypisać błąd
            return;
        }
    };

    if is_valid_queen_move(board, &piece, row, col, row_dest, col_dest) {
        let moved_piece = board[row][col].piece.take().unwrap();
        board[row_dest][col_dest].piece = Some(moved_piece);
        board[row][col].piece = None;

        // Możesz dodać print: "Ruch wykonany!"
    } else {
        // Możesz dodać print: "Nieprawidłowy ruch!"
    }
}

fn is_valid_queen_move(
    board: &[[Board; 8]; 8],
    piece: &Piece,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> bool {
    // --- 1. Jeśli końcowe pole ma własną figurę → NIE wolno wchodzić ---
    if let Some(target) = &board[to_row][to_col].piece {
        if target.color == piece.color {
            return false;
        }
    }

    let row_diff = (to_row as isize - from_row as isize).abs();
    let col_diff = (to_col as isize - from_col as isize).abs();

    let row_step = match to_row.cmp(&from_row) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    };

    let col_step = match to_col.cmp(&from_col) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    };

    // --- 2. Hetman może ruszać się jak wieża (prosto) i jak goniec (po skosie) ---
    let is_straight_move = from_row == to_row || from_col == to_col;
    let is_diagonal_move = row_diff == col_diff;

    if !is_straight_move && !is_diagonal_move {
        return false;
    }

    // --- 3. Sprawdzenie, czy droga jest wolna ---
    let mut r = from_row as isize + row_step;
    let mut c = from_col as isize + col_step;

    while r != to_row as isize || c != to_col as isize {
        if board[r as usize][c as usize].piece.is_some() {
            return false; // coś stoi po drodze
        }
        r += row_step;
        c += col_step;
    }

    true
}
