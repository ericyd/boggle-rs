use board::{Board, Piece};
use std::cmp::Ordering;
use std::fmt::{self, Formatter, Display};
use std::cmp::PartialEq;
use std::io::{BufRead, BufReader, Lines, Result};

// used in tests only
#[allow(unused_imports)]
use board;

pub struct Game {
    pub board: Board,
    pub player: Player,
    pub guesses: Guesses,
}

impl Game {
    pub fn add_guess(&mut self, word: String) {
        self.guesses.add_guess(word, &self.board)
    }
}

pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player { name }
    }
}

#[derive(Debug)]
pub struct Guess {
    word: String,
    score: usize,
}

impl Guess {
    pub fn new(word: String) -> Guess {
        Guess {
            score: Guess::calculate_score(word.len()),
            word: word,
        }
    }

    fn calculate_score(n: usize) -> usize {
        // 3 is the basic number by which scores are calculated
        let magic_number = 3;
        match n.cmp(&magic_number) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => n - magic_number,
        }
    }

    fn is_valid(&self, board: &Board) -> bool {
        // TODO: Check that word exists in dictionary
        // https://www.wordgamedictionary.com/twl06/download/twl06.txt

        // get vector of vector of &Piece
        // this should probably be made into its own struct or data type,
        // but this will suffice for proof of concept
        let mut collection: Vec<Vec<&Piece>> = vec![];

        // verify letters exist on board
        // if letter exists, add all instances to collection
        for letter in self.word.to_uppercase().chars() {
            match board.first_instance_of(letter) {
                Some(_x) => {
                    collection.push(board.get_all_instances_of(letter));
                    continue;
                }
                None => return false,
            };
        }

        is_valid_path(&collection, 0, None, &mut vec![])
    }
}

// tests all possible paths and returns true when/if one is valid
// since we don't actually need to know the path, this is greatly simplified
fn is_valid_path<'a>(
    collection: &Vec<Vec<&'a Piece>>,
    index: usize,
    prev_piece: Option<&Piece>,
    path: &mut Vec<&'a Piece>,
) -> bool {
    for piece in collection[index].iter() {
        // if piece is duplicate, not valid
        if path.contains(piece) {
            continue;
        }
        path.push(piece.clone());

        // not valid if piece is not a neighbor
        match prev_piece {
            Some(x) => {
                if !piece.is_neighbor(x) {
                    continue;
                }
            }
            None => (),
        }

        // search subsequent pieces if not at last piece
        if index < collection.len() - 1 {
            if is_valid_path(collection, index + 1, Some(piece), path) {
                return true;
            } else {
                // if this piece doesn't have a valid path,
                // remove piece and continue
                path.pop();
                continue;
            }
        }

        // if we've made it this far
        // then we are at the last piece and have a valid path!
        return true;
    }
    false
}


impl PartialEq for Guess {
    fn eq(&self, other: &Guess) -> bool {
        self.word == other.word
    }

    fn ne(&self, other: &Guess) -> bool {
        self.word != other.word
    }
}

impl Display for Guess {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}", self.word)
    }
}

#[derive(Debug)]
pub struct Guesses {
    invalid: Vec<Guess>,
    valid: Vec<Guess>,
    not_in_dict: Vec<Guess>,
    score: usize,
}

impl Guesses {
    pub fn new() -> Guesses {
        Guesses {
            invalid: vec![],
            valid: vec![],
            not_in_dict: vec![],
            score: 0,
        }
    }

    pub fn add_guess(&mut self, word: String, board: &Board) {
        let guess = Guess::new(word);
        // skip if duplicate word
        if self.valid.contains(&guess) {
            return;
        }
        // add to proper queue based on validity
        if guess.is_valid(board) {
            // if dictionary exists, check for existence
            // if no dictionary, then word is valid by default
            let word_upper = guess.word.to_uppercase();
            match board.dictionary {
                Some(lines) => {
                    match lines.map(|ref l| l.unwrap()).position(|line| line == word_upper) {
                        Some(_) => self.valid.push(guess),
                        None => self.not_in_dict.push(guess),
                    }
                }
                None => self.valid.push(guess),
            }
        } else {
            self.invalid.push(guess)
        }
    }
}

impl Display for Guesses {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "\nValid words\n=============\n")?;
        for guess in self.valid.iter() {
            write!(f, "{}\n", guess)?;
        }

        write!(f, "\nInvalid words\n=============\n")?;
        for guess in self.invalid.iter() {
            write!(f, "{}\n", guess)?;
        }

        write!(f, "\nNot in dictionary\n=============\n")?;
        for guess in self.not_in_dict.iter() {
            write!(f, "{}\n", guess)?;
        }

        // it appears that type annotations for sum are quite common, though not ideal
        // https://users.rust-lang.org/t/using-iterator-sum-in-an-expression/7126/5
        write!(
            f,
            "\nTotal Score: {:?}",
            self.valid.iter().map(|ref x| x.score).sum::<usize>()
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;
    #[test]
    fn guesses_add_guess_no_duplicates() {
        let mut my_guesses = Guesses::new();
        let my_board = Board::from(&vec!['T', 'E', 'S', 'T', 'R']);
        let my_string = String::from("test");
        my_guesses.add_guess(my_string, &my_board);
        let my_string = String::from("test");
        my_guesses.add_guess(my_string, &my_board);
        assert_eq!(my_guesses.invalid.len(), 0, "invalid is wrong length");
        assert_eq!(my_guesses.valid.len(), 1, "valid is wrong length");
    }

    #[test]
    fn guess_calculate_score() {
        let score = Guess::calculate_score(0);
        assert_eq!(score, 0);
        let score = Guess::calculate_score(2);
        assert_eq!(score, 0);
        let score = Guess::calculate_score(3);
        assert_eq!(score, 1);
        let score = Guess::calculate_score(4);
        assert_eq!(score, 1);
        let score = Guess::calculate_score(10);
        assert_eq!(score, 7);
    }

    #[test]
    fn guess_is_valid_letter_not_in_board() {
        let board = Board::from(&vec!['A', 'B', 'C', 'D', 'E']);
        let guess = Guess::new(String::from("testr"));
        assert_eq!(guess.is_valid(&board), false);
    }

    #[test]
    fn is_valid_path_single_element_true() {
        // trivial case, but when collection.len() == 1, a valid path always exists
        let piece = Piece::new('A', 0);
        let collection = vec![vec![&piece]];
        assert_eq!(
            is_valid_path(&collection, 0, None, &mut vec![]),
            true,
            "collection with 1 piece does not have valid path"
        );
    }

    #[test]
    fn is_valid_path_non_adjacent_pieces() {
        let piece1 = Piece::new('A', 0);
        let piece2 = Piece::new('A', 2);
        let collection = vec![vec![&piece1], vec![&piece2]];
        assert_eq!(
            is_valid_path(&collection, 0, None, &mut vec![]),
            false,
            "collection with non-adjacent pieces has valid path"
        );
    }

    #[test]
    fn is_valid_path_adjacent_pieces() {
        let piece1 = Piece::new('A', 0);
        let piece2 = Piece::new('A', 1);
        let collection = vec![vec![&piece1], vec![&piece2]];
        assert_eq!(
            is_valid_path(&collection, 0, None, &mut vec![]),
            true,
            "collection with adjacent pieces does not have valid path"
        );
    }

    #[test]
    fn is_valid_path_same_pieces() {
        let piece1 = Piece::new('A', 0);
        let piece2 = Piece::new('A', 1);
        let collection = vec![
            vec![&piece1],
            vec![&piece2],
            vec![&piece1] // should not be allowed to use same piece twice
        ];
        assert_eq!(
            is_valid_path(&collection, 0, None, &mut vec![]),
            false,
            "collection with duplicate pieces has valid path"
        );
    }

    #[test]
    fn is_valid_path_last_piece_wrong() {
        let piece1 = Piece::new('A', 0);
        let piece2 = Piece::new('A', 1);
        let piece3 = Piece::new('A', 20);
        let collection = vec![vec![&piece1], vec![&piece2], vec![&piece3]];
        assert_eq!(
            is_valid_path(&collection, 0, None, &mut vec![]),
            false,
            "collection with only partial path has full valid path"
        );
    }

    // cargo test is_valid_path_try_all_combinations -- --nocapture
    #[test]
    fn is_valid_path_try_all_combinations() {
        // should be able to try all possible combinations if collection has lots of invalid paths
        // yes, this test is quite ugly. I was trying to prove a point and it got out of hand
        let valid0 = Piece::new('A', 0);
        let valid5 = Piece::new('B', board::BOARD_DIMENSIONS);
        let valid6 = Piece::new('C', board::BOARD_DIMENSIONS * 2);
        let valid11 = Piece::new('D', board::BOARD_DIMENSIONS * 3);

        let invalid3 = Piece::new('E', board::BOARD_DIMENSIONS - 1);
        let invalid4 = Piece::new('F', board::BOARD_DIMENSIONS * 2 - 1);
        let invalid8 = Piece::new('G', board::BOARD_DIMENSIONS * 3 - 1);
        let invalid9 = Piece::new('H', board::BOARD_DIMENSIONS - 1);
        let invalid13 = Piece::new('I', board::BOARD_DIMENSIONS - 2);
        let invalid14 = Piece::new('J', board::BOARD_DIMENSIONS * 2 - 2);
        let invalid24 = Piece::new('K', board::BOARD_DIMENSIONS * 3 - 2);
        let invalid25 = Piece::new('L', board::BOARD_DIMENSIONS - 2);

        let collection = vec![
            vec![&invalid3, &invalid4, &valid0],
            vec![&invalid8, &invalid9, &valid5],
            vec![&invalid13, &invalid14, &valid6],
            vec![&invalid24, &invalid25, &valid11],
        ];
        assert_eq!(
            is_valid_path(&collection, 0, None, &mut vec![]),
            true,
            "collection with complicated path is valid"
        );
    }
}
