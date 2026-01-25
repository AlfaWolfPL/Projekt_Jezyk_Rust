mod pawn_logic;
mod knight_logic;
mod bishop_logic;
mod rook_logic;
mod queen_logic;
mod king_logic;
mod move_piece;

use phf::phf_map;
use std::io;
//use clearscreen;
static INDEX_ROW: phf::Map<&'static str,usize>=phf_map! {
    "a"=>0,
    "A"=>0,
    "b"=>1,
    "B"=>1,
    "c"=>2,
    "C"=>2,
    "d"=>3,
    "D"=>3,
    "e"=>4,
    "E"=>4,
    "f"=>5,
    "F"=>5,
    "g"=>6,
    "G"=>6,
    "h"=>7,
    "H"=>7,
};

//static ROW_INDEX: phf::Map<usize,&'static str>=phf_map!{
//    0=>"A",
//    1=>"B",
//    2=>"C",
//    3=>"D",
//    4=>"E",
//    5=>"F",
//    6=>"G",
//    7=>"H",
//};


#[derive(Copy,Clone,PartialEq)]

enum Color{
    Black,
    White,
}
#[derive(Copy,Clone)]
enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Copy,Clone)]
struct Piece{
    color: Color,
    piece_type: PieceType,
    is_moved: bool,
}
#[derive(Copy,Clone)]
struct Board{
    piece: Option<Piece>,
    //row: usize,
    //col: usize,
}



impl Board {
    fn new(row_n: usize, col_n: usize) -> Self {

            match row_n {
                0 => match col_n {
                    0|7 => Board{piece: Some(Piece {color: Color::White, piece_type: PieceType::Rook, is_moved: false })},
                    1|6 => Board{piece: Some(Piece { color: Color::White, piece_type: PieceType::Knight, is_moved: false })},
                    //1|6 =>Board { piece: None },
                    2|5 => Board{piece: Some(Piece { color: Color::White, piece_type: PieceType::Bishop, is_moved: false })},
                    //2|5 =>Board { piece: None },
                    3 => Board{piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen, is_moved: false })},
                    //3 =>Board { piece: None },
                    4 => Board{piece: Some(Piece { color: Color::White, piece_type: PieceType::King, is_moved: false })},
                    _ => Board { piece: None },
                }
                1=> match col_n{
                    0..=7 => Board{piece: Some(Piece { color: Color::White, piece_type: PieceType::Pawn, is_moved: false })},
                    _ => Board { piece: None },
                }
                6 => match col_n{
                    0..=7 => Board{piece: Some(Piece { color: Color::Black, piece_type: PieceType::Pawn, is_moved: false })},
                    _ => Board { piece: None },
                }
                7 => match col_n{
                    0|7 => Board{piece: Some(Piece { color: Color::Black, piece_type: PieceType::Rook, is_moved: false })},
                    1|6 => Board{piece: Some(Piece { color: Color::Black, piece_type: PieceType::Knight, is_moved: false })},
                    2|5 => Board{piece: Some(Piece { color: Color::Black, piece_type: PieceType::Bishop, is_moved: false })},
                    3 => Board{piece: Some(Piece { color: Color::Black, piece_type: PieceType::Queen, is_moved: false })},
                    4 => Board{piece: Some(Piece { color: Color::Black, piece_type: PieceType::King, is_moved: false })},
                    _ => Board { piece: None },
                },
                _ => Board { piece: None },
            }


    }
    fn display_char(&self) -> char {
        match &self.piece {
            Some(piece) => {
                match piece.piece_type {
                    PieceType::King => match piece.color {
                        Color::White => '♔',
                        Color::Black => '♚',
                    },
                    PieceType::Queen => match piece.color {
                        Color::White => '♕',
                        Color::Black => '♛',
                    },
                    PieceType::Rook => match piece.color {
                        Color::White => '♖',
                        Color::Black => '♜',
                    },
                    PieceType::Bishop => match piece.color {
                        Color::White => '♗',
                        Color::Black => '♝',
                    },
                    PieceType::Knight => match piece.color {
                        Color::White => '♘',
                        Color::Black => '♞',
                    },
                    PieceType::Pawn => match piece.color {
                        Color::White => '♙',
                        Color::Black => '♟',
                    },
                }
            }
            None => '·',
        }
    }
}

fn main() {
    let mut board: [[Board;8];8] =[[Board { piece: None, }; 8]; 8];
    for (row_idx, row_data) in board.iter_mut().enumerate() {
        for (col_idx, field) in row_data.iter_mut().enumerate() {
            *field = Board::new(row_idx, col_idx);
        }
    }
    let mut who_is:usize = 0;
    let mut start = true;
    while start {
        for row in (0..8).rev() {  // Odwróć, aby wyświetlić od góry (rząd 7) do dołu (rząd 0)
            for col in 0..8 {
                print!("{} ", board[row][col].display_char());
            }
            println!(); // nowa linia po każdym rzędzie
        }
        println!("Menu:");
        let color=match who_is{
            0 => "Biały",
            1 => "Czarny",
            _ => panic!()
        };
        println!("Ruch wykonuje: {}", color );
        println!("1. Wykonaj ruch");
        println!("0. zakończ grę");
        println!("Wybierz:");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Błąd odczytu");

        let dig_opt: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("To nie jest liczba! Spróbuj ponownie.");
                continue;
            }
        };

        match dig_opt {
            0 => start = exit_menu(),
            1 => moves(&mut board, who_is),
            _ => {
                println!("Nieznana opcja!");
                continue;
            }
        }

        who_is = switch_player(who_is);
        clearscreen::clear().unwrap();
    }
}

fn moves(board: &mut [[Board;8];8], who_is:usize) {
    println!("Podaj index bierki:");
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


    move_piece::move_piece(board, row_dest, col_dest, who_is);


}
fn switch_player(who_is:usize)->usize{
    match who_is {
        1=>0,
        0=>1,
        _ => panic!(),
    }
}

fn exit_menu() ->bool {
    false
}
