/// Define Board and Pieces
/// Boards and games have a 1:1 ratio
/// Boards are made up of 25 Pieces

use std::fmt::{self, Formatter, Display};
use rand::{thread_rng, Rng};
use rand::distributions::{Sample, Range};

// representation of the playing board
#[derive(Debug)]
pub struct Board {
    pieces: Vec<Piece>,
    include_borders: bool,
}

// 0-indexed row/col Piece on the Board
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    letter: char,
    row: i32,
    col: i32,
}

const VOWELS: [char; 6] = ['A', 'E', 'I', 'O', 'U', 'Y'];
const CONSONANTS_UNFRIENDLY: [char; 6] = ['J', 'K', 'Q', 'V', 'X', 'Z'];
const CONSONANTS_FRIENDLY: [char; 14] = [
    'B',
    'C',
    'D',
    'F',
    'G',
    'H',
    'L',
    'M',
    'N',
    'P',
    'R',
    'S',
    'T',
    'W',
];

impl Board {
    pub fn new() -> Board {
        let mut pieces = Vec::new();

        // generate letters
        // choose vowels ~50% of the time
        // sprinkle some unfriendly consonants with the friendly consonants
        let mut rng = thread_rng();
        let mut vowel_range = Range::new(0usize, VOWELS.len());
        let mut consonant_unfriendly_range = Range::new(0usize, CONSONANTS_UNFRIENDLY.len());
        let mut consonant_friendly_range = Range::new(0usize, CONSONANTS_FRIENDLY.len());
        let mut unfriendly_range = Range::new(0usize, 10usize);
        for i in 0..25 {
            let letter = if rng.gen() {
                VOWELS[vowel_range.sample(&mut rng)]
            } else if unfriendly_range.sample(&mut rng) < 1 {
                CONSONANTS_UNFRIENDLY[consonant_unfriendly_range.sample(&mut rng)]
            } else {
                CONSONANTS_FRIENDLY[consonant_friendly_range.sample(&mut rng)]
            };
            pieces.push(Piece::new(letter, i));
        }

        Board {
            pieces: pieces,
            include_borders: false,
        }
    }

    // used for testing, never used in actual binary
    #[allow(dead_code)]
    pub fn from(letters: &Vec<char>) -> Board {
        let mut pieces = Vec::new();
        // ideally would convert letter to uppercase,
        // but the to_uppercase method is not very user friendly
        // tried: .map(|&x| x.to_uppercase().collect())
        for (i, l) in letters.iter().enumerate() {
            pieces.push(Piece::new(*l, i));
        }
        Board {
            pieces: pieces,
            include_borders: false,
        }
    }

    // returns the first instance of a letter
    pub fn first_instance_of(&self, letter: char) -> Option<usize> {
        self.pieces.iter().position(|ref x| x.letter == letter)
    }

    pub fn get_all_instances_of(&self, letter: char) -> Vec<&Piece> {
        self.pieces.iter().filter(|&x| x.letter == letter).collect()
    }
}

impl Display for Board {
    // print all pieces sequentially
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.include_borders {
            // write top border
            for _i in 0..21 {
                write!(f, "-")?;
            }
        }
        write!(f, "\n")?;
        for piece in self.pieces.iter() {
            // piece includes left border and padding
            if self.include_borders {
                write!(f, "|{}", piece)?;
            } else {
                write!(f, "{}", piece)?;
            }
            // write right-border
            if piece.col != 0 && piece.col % 4 == 0 {
                if self.include_borders {
                    write!(f, "|\n")?;
                    for _i in 0..21 {
                        write!(f, "-")?;
                    }
                } else {
                    write!(f, "\n")?;
                }
                write!(f, "\n")?;
            }
        }
        write!(f, "\n")
    }
}

impl Piece {
    pub fn new(letter: char, index: usize) -> Piece {
        let (row, col) = idx(index);
        Piece { letter, row, col }
    }

    pub fn is_neighbor(&self, p: &Piece) -> bool {
        // same piece is not neighbor
        if self.row == p.row && self.col == p.col {
            return false;
        }
        // if no more than 1 row and column away
        if (self.row - p.row).abs() <= 1 && (self.col - p.col).abs() <= 1 {
            return true;
        }
        false
    }
}

impl Display for Piece {
    // each piece is a letter surrounded by a 1-char margin of whitespace
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, " {} ", self.letter)
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Piece) -> bool {
        self.letter == other.letter && self.row == other.row && self.col == other.col
    }
    fn ne(&self, other: &Piece) -> bool {
        self.letter != other.letter || self.row != other.row || self.col != other.col
    }
}

// i should be 0-indexed too
// e.g. first row indices are 0-4, 5-9, 10-14, 15-19, 20-24
fn idx(i: usize) -> (i32, i32) {
    // These could/should be defined in a more public space?
    let i: i32 = i as i32;
    let num_cols = 5;
    let num_rows = 5;
    (i / num_rows, i % num_cols)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_gets_the_right_index() {
        let expected = (0, 3);
        let actual = idx(3);
        assert_eq!(actual, expected);

        let expected = (0, 4);
        let actual = idx(4);
        assert_eq!(actual, expected);

        let expected = (1, 0);
        let actual = idx(5);
        assert_eq!(actual, expected);

        let expected = (1, 4);
        let actual = idx(9);
        assert_eq!(actual, expected);

        let expected = (4, 4);
        let actual = idx(24);
        assert_eq!(actual, expected);
    }
}
