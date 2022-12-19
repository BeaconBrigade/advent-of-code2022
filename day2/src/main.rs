fn main() {
    let input = include_str!("input.txt");
    let game = Game::parse(input);
    println!("points: {}", game.points())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(input: &str) -> Self {
        match input {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            c => panic!("Invalid input: {:?}", c),
        }
    }

    fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, other: &Self) -> usize {
        use Move::{Paper, Rock, Scissors};

        let res = match (self, other) {
            (a, b) if a == b => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Scissors) => 0,
            (Paper, Rock) => 6,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            _ => unreachable!("the matching cases are covered by the first pattern"),
        };
        res + self.points()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ending {
    Win,
    Loss,
    Tie,
}

impl Ending {
    fn parse(input: &str) -> Self {
        match input {
            "X" => Self::Loss,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            c => panic!("Invalid input: {:?}", c),
        }
    }

    fn into_move(self, other: Move) -> Move {
        use Move::{Paper, Rock, Scissors};
        use Ending::{Win, Loss, Tie};

        match (self, other) {
            (Tie, m) => m,
            (Win, Rock) => Paper,
            (Loss, Rock) => Scissors,
            (Win, Paper) => Scissors,
            (Loss, Paper) => Rock,
            (Win, Scissors) => Rock,
            (Loss, Scissors) => Paper,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    pub moves: Vec<(Move, Ending)>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut moves = vec![];
        for line in input.trim().lines() {
            let (left, right) = line
                .split_once(' ')
                .map(|(l, r)| (Move::parse(l), Ending::parse(r)))
                .unwrap();
            moves.push((left, right));
        }
        Self { moves }
    }

    fn points(&self) -> usize {
        self.moves.iter().map(|(l, r)| r.into_move(*l).play(l)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_paper() {
        let input = "A Y\nB X\nC Z";
        let game = Game::parse(input);
        assert_eq!(game.points(), 12);
    }
}
