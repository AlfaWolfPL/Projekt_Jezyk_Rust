use std::io;
use crate::{Board, Color, Piece, INDEX_ROW};

pub(crate) fn move_knight(board: &mut [[Board; 8]; 8], piece: Piece, row: usize, col: usize) {
    // Info pomocnicze
    println!("Wybrano skoczka. Podaj ruch w formacie, np. C3:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu");

    let input = input.trim().to_uppercase();
    let mut chars = input.chars();
    let col_char = chars.next().unwrap_or(' ');
    let row_char = chars.next().unwrap_or('0');

    // Parsowanie kolumny (A-H)
    let mut binding = [0; 4];
    let col_str = col_char.encode_utf8(&mut binding);
    let col_dest = *INDEX_ROW.get(col_str).expect("Nieprawidłowa kolumna! Użyj A-H");

    // Parsowanie wiersza (1-8)
    let row_dest = match row_char.to_digit(10) {
        Some(num) if (1..=8).contains(&num) => (num - 1) as usize,
        _ => {
            println!("Nieprawidłowy wiersz! Użyj 1-8");
            return;
        }
    };

    // Walidacja ruchu skoczka
    if is_valid_knight_move(board, &piece, row, col, row_dest, col_dest) {
        let mut moved_piece = board[row][col].piece.take().unwrap();
        moved_piece.is_moved = true;
        board[row_dest][col_dest].piece = Some(moved_piece);
        board[row][col].piece = None;
        println!("Ruch wykonany!");
    } else {
        println!("Nieprawidłowy ruch!");
    }
}

fn is_valid_knight_move(
    board: &[[Board; 8]; 8],
    piece: &Piece,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> bool {
    // Różnice ruchu
    let row_diff = isize::abs(to_row as isize - from_row as isize);
    let col_diff = isize::abs(to_col as isize - from_col as isize);

    // Skoczek musi wykonać ruch w kształcie L:
    // 2 w jednym kierunku i 1 w drugim
    let valid_shape = (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2);

    if !valid_shape {
        return false;
    }

    // Sprawdzamy, czy na polu docelowym nie stoi figura tego samego koloru
    if let Some(target_piece) = &board[to_row][to_col].piece {
        if target_piece.color == piece.color {
            return false;
        }
    }

    true
}
