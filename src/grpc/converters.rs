use crate::concurrent;
use crate::game;
use crate::grpc::server::dominoes;
use crate::grpc::server::dominoes::message::Content;

const LEFT: i32 = 0;
const RIGHT: i32 = 1;

fn to_init_msg(pieces: &Vec<game::Domino>, number: usize) -> dominoes::Message {
    let hand = pieces
        .iter()
        .map(|domino| dominoes::Piece {
            up: domino.0,
            down: domino.1,
        })
        .collect();

    dominoes::Message {
        content: Some(Content::Init(dominoes::Init {
            hand,
            number: number as i32,
        })),
    }
}

fn to_update_msg(domino: &game::Domino, position: i32) -> dominoes::Message {
    dominoes::Message {
        content: Some(Content::Update(dominoes::Update {
            piece: Some(dominoes::Piece {
                up: domino.0,
                down: domino.1,
            }),
            position,
            turn: 0,
        })),
    }
}

pub fn to_proto(msg: &concurrent::Message) -> dominoes::Message {
    match *msg {
        concurrent::Message::Init((pieces, number)) => to_init_msg(pieces, number),

        concurrent::Message::YourTurn => dominoes::Message {
            content: Some(Content::YouTurn(dominoes::YourTurn {})),
        },

        concurrent::Message::Update(game::Update::Left(domino)) => to_update_msg(&domino, LEFT),

        concurrent::Message::Update(game::Update::Right(domino)) => to_update_msg(&domino, RIGHT),

        concurrent::Message::Update(game::Update::Skip) => dominoes::Message {
            content: Some(Content::Skip(dominoes::Skip {})),
        },
    }
}

pub fn move_to_proto(move_: &game::Move) -> dominoes::Message {
    let (side, player_number , piece_position) = match move_ {
        game::Move::Left(player, piece) => (LEFT, *player as i32, *piece as i32),
        game::Move::Right(player, piece) => (RIGHT, *player as i32, *piece as i32),
    };

    dominoes::Message {
        content: Some(Content::Move(
            dominoes::Move{
                side,
                piece_position,
                player_number
            }
        )),
    }
}

pub fn to_move(msg: &dominoes::Message) -> Option<game::Move> {
    if let Some(Content::Move(move_)) = &msg.content {
        let player_pos = move_.player_number as usize;
        let piece_num = move_.piece_position as usize;

        return match move_.side {
            LEFT => Some(game::Move::Left(player_pos, piece_num)),
            _ => Some(game::Move::Right(player_pos, piece_num)),
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter() { // to be finished, I'm in a hurry now
        let x = concurrent::Message::Update(game::Update::Left(game::Domino(5, 5)));

        let y = to_proto(&x);

        println!("|{:?}|", y);
    }
}
