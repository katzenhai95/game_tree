use game_tree::game_tree::{GameTree, SituationOps};

use core::fmt;
use std::{array, cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TicTacToePlayer {
    Offensive,
    Defensive,
}

impl TicTacToePlayer {
    fn swap(&self) -> TicTacToePlayer {
        match self {
            Self::Offensive => Self::Defensive,
            Self::Defensive => Self::Offensive,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TicTacToeResult {
    Unknown,
    Win,
    Lose,
    Tie,
}

enum TicTacToeError {
    IllegalMove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TicTacToeStatus(Option<TicTacToePlayer>);

impl From<Option<TicTacToePlayer>> for TicTacToeStatus {
    fn from(status: Option<TicTacToePlayer>) -> Self {
        Self(status)
    }
}

impl fmt::Display for TicTacToeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(TicTacToePlayer::Offensive) => {
                write!(f, "O")
            }
            Some(TicTacToePlayer::Defensive) => {
                write!(f, "X")
            }
            None => {
                write!(f, " ")
            }
        }
    }
}

#[derive(Debug)]
struct TicTacToeSituation {
    player: TicTacToePlayer,
    current_move_player: TicTacToePlayer,
    situation: [TicTacToeStatus; 9],
}

impl TicTacToeSituation {
    fn new(player: TicTacToePlayer, current_move_player: TicTacToePlayer) -> Self {
        Self {
            player,
            situation: array::from_fn(|_| None.into()),
            current_move_player,
        }
    }

    fn check_result(&self) -> TicTacToeResult {
        let check_order = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for single_check in check_order {
            if let (
                TicTacToeStatus(Some(player1)),
                TicTacToeStatus(Some(player2)),
                TicTacToeStatus(Some(player3)),
            ) = (
                &self.situation[single_check[0]],
                &self.situation[single_check[1]],
                &self.situation[single_check[2]],
            ) {
                if player1 == player2 && player2 == player3 {
                    if player1 == &self.player {
                        return TicTacToeResult::Win;
                    } else {
                        return TicTacToeResult::Lose;
                    }
                }
            }
        }
        return TicTacToeResult::Unknown;
    }

    fn proc_move(&mut self, next_move: &TicTacToeMove) {
        self.situation[next_move.pos] = Some(next_move.player.clone()).into();
        self.current_move_player = match self.current_move_player {
            TicTacToePlayer::Offensive => TicTacToePlayer::Defensive,
            TicTacToePlayer::Defensive => TicTacToePlayer::Offensive,
        }
    }
}

#[derive(Debug)]
struct TicTacToeMove {
    pos: usize,
    player: TicTacToePlayer,
}

impl TicTacToeMove {
    fn new(pos: usize, player: TicTacToePlayer) -> Self {
        TicTacToeMove { pos, player }
    }
}

fn tic_tac_toe_get_input() -> Option<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", line);
    match <usize as FromStr>::from_str(line.trim()) {
        Ok(res) => match res.cmp(&9) {
            Ordering::Less => Some(res),
            _ => None,
        },
        Err(_) => None,
    }
}

fn tic_tac_toe_check_input(
    cur_situation: &TicTacToeSituation,
    next_move: usize,
) -> Result<(), TicTacToeError> {
    match cur_situation.situation[next_move].0 {
        None => Ok(()),
        _ => Err(TicTacToeError::IllegalMove),
    }
}

fn tic_tac_toe_display(cur_situation: &TicTacToeSituation) {
    println!(
        "Current situation:\n[0]{} [1]{} [2]{}\n[3]{} [4]{} [5]{}\n[6]{} [7]{} [8]{}",
        cur_situation.situation[0],
        cur_situation.situation[1],
        cur_situation.situation[2],
        cur_situation.situation[3],
        cur_situation.situation[4],
        cur_situation.situation[5],
        cur_situation.situation[6],
        cur_situation.situation[7],
        cur_situation.situation[8],
    );
}

fn tic_tac_toe() -> TicTacToeResult {
    let mut players: [Box<dyn PlayerOps>; 2] = [
        Box::new(ComPlayer::new(&TicTacToePlayer::Offensive)),
        Box::new(ComPlayer::new(&TicTacToePlayer::Offensive)),
    ]; // offensive, defensive
    let player_type = [TicTacToePlayer::Offensive, TicTacToePlayer::Defensive];
    for (i, player) in players.iter_mut().enumerate() {
        println!("Choose player {} type C/H: ", i);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "C" | "c" => *player = Box::new(ComPlayer::new(&player_type[i])),
            _ => *player = Box::new(HumanPlayer::new(&player_type[i])),
        }
    }
    let mut round = 0;
    let mut cur_player = 0;
    let mut cur_situation =
        TicTacToeSituation::new(TicTacToePlayer::Offensive, TicTacToePlayer::Offensive);
    println!("Begin!");
    tic_tac_toe_display(&cur_situation);
    loop {
        let next_move = players[cur_player].as_ref().get_move();
        for player in &mut players {
            player.proc_move(&next_move);
        }
        cur_situation.proc_move(&next_move);
        round += 1;
        cur_player = 1 - cur_player;
        let res = cur_situation.check_result();
        println!("\n\n\n\nRound {}", round);
        tic_tac_toe_display(&cur_situation);
        if res == TicTacToeResult::Win {
            println!("Offensive Win!");
            break TicTacToeResult::Win;
        } else if res == TicTacToeResult::Lose {
            println!("Defensive Win!");
            break TicTacToeResult::Lose;
        } else if round == 9 {
            println!("Tie!");
            break TicTacToeResult::Tie;
        }
    }
}

impl crate::SituationOps for TicTacToeSituation {
    type Move = TicTacToeMove;
    fn calc_cost(&self) -> i32 {
        match self.check_result() {
            TicTacToeResult::Win => 1,
            TicTacToeResult::Lose => -1,
            _ => 0,
        }
    }

    fn get_avilable_move(&self) -> Vec<Self::Move> {
        let mut next_move: Vec<Self::Move> = vec![];
        for (pos, pos_player) in self.situation.iter().enumerate() {
            if pos_player.0 == None {
                next_move.push(Self::Move::new(pos, self.current_move_player.clone()));
            }
        }
        next_move
    }

    fn proc_move(&mut self, next_move: &Self::Move) {
        self.situation[next_move.pos] = Some(next_move.player.clone()).into();
        self.current_move_player = self.current_move_player.swap();
    }

    fn with_move(&self, next_move: &Self::Move) -> Self {
        let mut new_situation = TicTacToeSituation {
            player: self.player.clone(),
            current_move_player: self.current_move_player.swap(),
            situation: self.situation.clone(),
        };
        new_situation.situation[next_move.pos] = Some(next_move.player.clone()).into();
        new_situation
    }
}

trait PlayerOps {
    fn get_move(&self) -> TicTacToeMove;
    fn proc_move(&mut self, next_move: &TicTacToeMove);
}

struct HumanPlayer {
    situation: TicTacToeSituation,
}

impl PlayerOps for HumanPlayer {
    fn get_move(&self) -> TicTacToeMove {
        let pos = loop {
            print!("Input next move: ");
            if let Some(pos) = tic_tac_toe_get_input() {
                if let Ok(()) = tic_tac_toe_check_input(&self.situation, pos) {
                    break pos;
                }
            }
            println!("invalid input");
        };
        TicTacToeMove {
            pos: pos,
            player: self.situation.player.clone(),
        }
    }

    fn proc_move(&mut self, next_move: &TicTacToeMove) {
        self.situation.proc_move(next_move);
    }
}

impl HumanPlayer {
    fn new(player: &TicTacToePlayer) -> Self {
        Self {
            situation: TicTacToeSituation::new(player.clone(), TicTacToePlayer::Offensive),
        }
    }
}

struct ComPlayer {
    game_tree: GameTree<TicTacToeSituation>,
}

impl PlayerOps for ComPlayer {
    fn get_move(&self) -> TicTacToeMove {
        match self.game_tree.get_next_move() {
            Some(next_move) => next_move,
            None => panic!(),
        }
    }

    fn proc_move(&mut self, next_move: &TicTacToeMove) {
        self.game_tree.proc_move(next_move);
    }
}

impl ComPlayer {
    fn new(player: &TicTacToePlayer) -> Self {
        Self {
            game_tree: GameTree::new(
                9,
                TicTacToeSituation::new(player.clone(), TicTacToePlayer::Offensive),
            ),
        }
    }
}

fn main() {
    loop {
        tic_tac_toe();
    }
}
