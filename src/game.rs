#![allow(unused)]

use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Domino(pub i32, pub i32);

impl Domino {
    pub fn match_right(&self, number: i32) -> Option<Domino> {
        match self {
            Domino(x, _) if *x == number => Some(*self),
            Domino(_, y) if *y == number => Some(self.reverse()),
            _ => None,
        }
    }

    pub fn match_left(&self, number: i32) -> Option<Domino> {
        match self {
            Domino(x, _) if *x == number => Some(self.reverse()),
            Domino(_, y) if *y == number => Some(*self),
            _ => None,
        }
    }

    pub fn reverse(&self) -> Domino {
        Domino(self.1, self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub players: [Vec<Domino>; 4],
    pub board: Vec<Domino>,
    pub next: i32,
    n_players: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Left(usize, usize),
    Right(usize, usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Update {
    Skip,
    Left(Domino),
    Right(Domino)
}

impl Move {
    pub fn parse_move(string: &str, player: usize) -> Option<Move> {
        let parts = string.split_whitespace().collect::<Vec<_>>();
        let slice = parts.as_slice();
        match slice {
            [side @ "left" | side @ "right", pos] => {
                let pos_: usize = pos.parse().ok()?;

                if *side == "left" {
                    Some(Move::Left(player, pos_))
                } else {
                    Some(Move::Right(player, pos_))
                }
            }
            _ => None,
        }
    }

    pub fn parse(string: &str) -> Option<Move> {
        let parts = string.split_whitespace().collect::<Vec<_>>();
        let slice = parts.as_slice();

        match slice {
            [side @ "left" | side @ "right", player, pos] => {
                let player_: usize = player.parse().ok()?;
                let pos_: usize = pos.parse().ok()?;

                if *side == "left" {
                    Some(Move::Left(player_, pos_))
                } else {
                    Some(Move::Right(player_, pos_))
                }
            }

            _ => None,
        }
    }

    fn unpack(&self) -> (usize, usize) {
        match *self {
            Move::Left(x, y) => (x, y),
            Move::Right(x, y) => (x, y),
        }
    }
}

impl Game {
    fn shuffled_pieces() -> Vec<Domino> {
        let mut rng = thread_rng();
        let mut pieces = (0..7)
            .flat_map(|i| (0..(i + 1)).map(move |j| Domino(i, j)))
            .collect::<Vec<Domino>>();
        pieces.shuffle(&mut rng);
        pieces
    }

    pub fn new(n_players: i32) -> Game {
        let pieces = Game::shuffled_pieces();

        let players = match n_players {
            2 => [
                pieces[0..14].to_vec(),
                pieces[14..].to_vec(),
                vec![],
                vec![],
            ],
            3 => [
                pieces[0..9].to_vec(),
                pieces[9..18].to_vec(),
                pieces[18..27].to_vec(),
                vec![],
            ],
            _ => [
                pieces[0..7].to_vec(),
                pieces[7..14].to_vec(),
                pieces[14..21].to_vec(),
                pieces[21..].to_vec(),
            ],
        };

        let game = Game {
            players,
            board: Vec::new(),
            next: 0,
            n_players,
        };

        game
    }

    pub fn play(&mut self, move_: &Move) -> Result<Update> {
        let update =  self.make_move(move_)?;
        self.incr_player();
        
        Ok(update)
    }

    fn incr_player(&mut self) {
        self.next += 1;

        if self.next == 4 {
            self.next = 0;
        }
    }

    fn make_move(&mut self, move_: &Move) -> Result<Update> {
        if self.board.is_empty() {
            let (player_num, piece_pos) = move_.unpack();
            let piece = self.players[player_num].remove(piece_pos);
            self.board.push(piece);
            return Ok(Update::Left(piece));
        }

        match *move_ {
            Move::Left(player_num, piece_pos) => self.play_left(player_num, piece_pos),
            Move::Right(player_num, piece_pos) => self.play_right(player_num, piece_pos),
        }
    }

    fn play_left(&mut self, player_num: usize, piece_pos: usize) -> Result<Update> {
        let piece_from_board = self.board[0];
        let piece_to_play = self.players[player_num][piece_pos];

        let piece_to_play = piece_to_play.match_left(piece_from_board.0);

        if let Some(piece) = piece_to_play {
            self.board.insert(0, piece);
            self.players[player_num].remove(piece_pos);
            return Ok(Update::Left(piece));
        }

        Err(anyhow!("Invalid move".to_string()))
    }

    fn play_right(&mut self, player_num: usize, piece_pos: usize) -> Result<Update> {
        let piece_from_board = self.board[self.board.len() - 1];
        let piece_to_play = self.players[player_num][piece_pos];

        let piece_to_play = piece_to_play.match_right(piece_from_board.1);

        if let Some(piece) = piece_to_play {
            self.board.push(piece);
            self.players[player_num].remove(piece_pos);
            return Ok(Update::Right(piece));
        }

        Err(anyhow!("Invalid move".to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_match() {
        let a = Domino(1, 2);

        let b = a.match_left(2);
        let c = a.match_right(2);
        let d = a.match_right(3);

        assert_eq!(Some(Domino(1, 2)), b);

        assert_eq!(Some(Domino(2, 1)), c);

        assert_eq!(None, d)
    }

    #[test]
    fn test_play_move_left() {
        let mut game = Game {
            players: [
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1), Domino(6, 4)],
                vec![Domino(5, 5), Domino(6, 6), Domino(1, 4)],
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1)],
                vec![Domino(3, 3), Domino(5, 6), Domino(2, 5), Domino(1, 1)],
            ],
            board: vec![Domino(3, 4), Domino(4, 2)],
            next: 0,
            n_players: 4,
        };

        let result = game.make_move(&Move::Left(0, 2));

        if let Err(_) = result {
            assert!(false);
        }

        let expected = Game {
            players: [
                vec![Domino(1, 2), Domino(2, 2), Domino(6, 4)],
                vec![Domino(5, 5), Domino(6, 6), Domino(1, 4)],
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1)],
                vec![Domino(3, 3), Domino(5, 6), Domino(2, 5), Domino(1, 1)],
            ],
            board: vec![Domino(1, 3), Domino(3, 4), Domino(4, 2)],
            next: 0,
            n_players: 4,
        };

        assert_eq!(expected, game);
    }

    #[test]
    fn test_play_move_right() {
        let mut game = Game {
            players: [
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1), Domino(6, 4)],
                vec![Domino(5, 5), Domino(6, 6), Domino(1, 4)],
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1)],
                vec![Domino(3, 3), Domino(5, 6), Domino(2, 5), Domino(1, 1)],
            ],
            board: vec![Domino(3, 4), Domino(4, 2)],
            next: 0,
            n_players: 4,
        };

        let result = game.make_move(&Move::Right(0, 0));

        if let Err(_) = result {
            assert!(false);
        }

        let expected = Game {
            players: [
                vec![Domino(2, 2), Domino(3, 1), Domino(6, 4)],
                vec![Domino(5, 5), Domino(6, 6), Domino(1, 4)],
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1)],
                vec![Domino(3, 3), Domino(5, 6), Domino(2, 5), Domino(1, 1)],
            ],
            board: vec![Domino(3, 4), Domino(4, 2), Domino(2, 1)],
            next: 0,
            n_players: 4,
        };

        assert_eq!(expected, game);
    }

    #[test]
    fn test_play_moves() {
        let mut game = Game {
            players: [
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1), Domino(6, 4)],
                vec![Domino(5, 5), Domino(6, 6), Domino(1, 4)],
                vec![Domino(3, 3), Domino(5, 6), Domino(2, 5), Domino(1, 1)],
                vec![],
            ],
            board: vec![Domino(3, 4), Domino(4, 2)],
            next: 0,
            n_players: 4,
        };

        let moves = [
            Move::Left(0, 2),
            Move::Left(1, 2),
            Move::Right(2, 2),
            Move::Left(0, 2),
        ];

        for game_move in moves.iter() {
            let _ = game.make_move(game_move);
        }

        let expected = Game {
            players: [
                vec![Domino(1, 2), Domino(2, 2)],
                vec![Domino(5, 5), Domino(6, 6)],
                vec![Domino(3, 3), Domino(5, 6), Domino(1, 1)],
                vec![],
            ],
            board: vec![
                Domino(6, 4),
                Domino(4, 1),
                Domino(1, 3),
                Domino(3, 4),
                Domino(4, 2),
                Domino(2, 5),
            ],
            next: 0,
            n_players: 4,
        };

        assert_eq!(expected, game);
    }

    #[test]
    fn test_empty_board() {
        let mut game = Game {
            players: [
                vec![Domino(1, 2), Domino(2, 2), Domino(3, 1), Domino(6, 4)],
                vec![],
                vec![],
                vec![],
            ],
            board: Vec::new(),
            next: 0,
            n_players: 4,
        };

        let expected = Game {
            players: [
                vec![Domino(1, 2), Domino(3, 1), Domino(6, 4)],
                vec![],
                vec![],
                vec![],
            ],
            board: vec![Domino(2, 2)],
            next: 0,
            n_players: 4,
        };

        let result = game.make_move(&Move::Left(0, 1));

        println!("{:?}", result);

        if let Err(_) = result {
            assert!(false);
        }

        assert_eq!(expected, game);
    }

    #[test]
    fn parse_moves() {
        let moves: [(&str, Move); 4] = [
            ("left 1 2", Move::Left(1, 2)),
            ("right 1 2", Move::Right(1, 2)),
            ("right 0 0", Move::Right(0, 0)),
            ("left 1 1", Move::Left(1, 1)),
        ];

        for tuple in moves {
            let (string, result) = tuple;

            assert_eq!(result, Move::parse(string).unwrap());
        }
    }
}
