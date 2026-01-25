use std::io;
use crate::{Board, Piece, PieceType, INDEX_ROW};

pub(crate) fn move_king(board: &mut [[Board; 8]; 8], piece: Piece, row: usize, col: usize) {
    println!("Wybrano Króla. Podaj ruch (np. E2) lub pole roszady (np. C1/G1):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Błąd odczytu");

    let input = input.trim().to_uppercase();
    let mut chars = input.chars();
    let col_char = chars.next().unwrap_or(' ');
    let row_char = chars.next().unwrap_or('0');

    // Parsowanie kolumny (A-H)
    let mut binding = [0; 4];
    let col_str = col_char.encode_utf8(&mut binding);
    // Zakładam, że INDEX_ROW to HashMap lub statyczna tablica mapująca "A"->0, itd.
    let col_dest = match INDEX_ROW.get(col_str) {
        Some(&val) => val,
        None => {
            println!("Nieprawidłowa kolumna! Użyj A-H");
            return;
        }
    };

    // Parsowanie wiersza (1-8)
    let row_dest = match row_char.to_digit(10) {
        Some(num) if (1..=8).contains(&num) => (num - 1) as usize,
        _ => {
            println!("Nieprawidłowy wiersz! Użyj 1-8");
            return;
        }
    };

    // Oblicz różnice odległości
    let row_diff = (row_dest as isize - row as isize).abs();
    let col_diff = (col_dest as isize - col as isize).abs();

    // --- LOGIKA ROSZADY (Ruch o 2 pola w poziomie w tym samym wierszu) ---
    if row_diff == 0 && col_diff == 2 {
        // Sprawdzamy, czy roszada jest logicznie możliwa (pola puste, brak ruchu figur)
        if can_castle(board, &piece, row, col, col_dest) {
            println!("Wykryto możliwą roszadę. Czy chcesz ją wykonać? (t/n)");

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).expect("Błąd odczytu");

            if confirm.trim().to_lowercase() == "t" {
                perform_castling(board, row, col, col_dest);
                println!("Roszada wykonana!");
                return;
            } else {
                println!("Anulowano roszadę.");
                return;
            }
        } else {
            println!("Roszada niemożliwa (figury ruszone lub przeszkody na drodze)!");
            return;
        }
    }

    // --- LOGIKA STANDARDOWA (Ruch o 1 pole) ---
    if is_valid_king_move(board, &piece, row, col, row_dest, col_dest) {
        // Przeniesienie Króla
        let mut moved_piece = board[row][col].piece.take().unwrap();
        moved_piece.is_moved = true; // Oznaczamy, że król się ruszył
        board[row_dest][col_dest].piece = Some(moved_piece);
        board[row][col].piece = None;
        println!("Ruch wykonany!");
    } else {
        println!("Nieprawidłowy ruch Króla!");
    }
}

// Funkcja sprawdzająca standardowy ruch o 1 pole
fn is_valid_king_move(
    board: &[[Board; 8]; 8],
    piece: &Piece,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> bool {
    let row_diff = (to_row as isize - from_row as isize).abs();
    let col_diff = (to_col as isize - from_col as isize).abs();

    // Król porusza się max o 1 pole w każdą stronę
    if row_diff > 1 || col_diff > 1 {
        return false;
    }

    // Sprawdzamy czy pole docelowe nie jest zajęte przez własną figurę
    if let Some(target_piece) = &board[to_row][to_col].piece
        && target_piece.color == piece.color {
        return false;
    }

    true
}

// Funkcja weryfikująca warunki roszady
fn can_castle(
    board: &[[Board; 8]; 8],
    king: &Piece,
    row: usize,
    from_col: usize,
    to_col: usize,
) -> bool {
    // 1. Czy król się już ruszył?
    if king.is_moved {
        return false;
    }

    // Określenie kierunku i pozycji wieży
    // Jeśli to_col > from_col (np. E->G), to krótka roszada (wieża po prawej, kolumna 7)
    // Jeśli to_col < from_col (np. E->C), to długa roszada (wieża po lewej, kolumna 0)
    let is_kingside = to_col > from_col;
    let rook_col = if is_kingside { 7 } else { 0 };

    // 2. Pobranie wieży i sprawdzenie czy istnieje oraz czy się ruszyła
    let rook_piece = match &board[row][rook_col].piece {
        Some(p) if matches!(p.piece_type, PieceType::Rook) => p,
        _ => return false, // Brak wieży w rogu
    };

    if rook_piece.is_moved {
        return false;
    }

    // 3. Sprawdzenie czy pola pomiędzy Królem a Wieżą są puste
    // Zakres kolumn do sprawdzenia (bez pozycji króla i wieży)
    let (start, end) = if is_kingside {
        (from_col + 1, 6) // Dla krótkiej: F(5), G(6)
    } else {
        (1, from_col - 1) // Dla długiej: B(1), C(2), D(3)
    };

    for c in start..=end {
        if board[row][c].piece.is_some() {
            return false; // Droga zablokowana
        }
    }

    true
}

// Funkcja wykonująca fizyczną roszadę na planszy
fn perform_castling(board: &mut [[Board; 8]; 8], row: usize, king_from: usize, king_to: usize) {
    let is_kingside = king_to > king_from;

    // 1. Przesunięcie Króla
    let mut king = board[row][king_from].piece.take().unwrap();
    king.is_moved = true;
    board[row][king_to].piece = Some(king);

    // 2. Przesunięcie Wieży
    let rook_from_col = if is_kingside { 7 } else { 0 };
    // Wieża ląduje obok Króla (po jego drugiej stronie)
    // Krótka: Król na G(6), Wieża idzie na F(5)
    // Długa: Król na C(2), Wieża idzie na D(3)
    let rook_to_col = if is_kingside { king_to - 1 } else { king_to + 1 };

    let mut rook = board[row][rook_from_col].piece.take().unwrap();
    rook.is_moved = true;
    board[row][rook_to_col].piece = Some(rook);
}