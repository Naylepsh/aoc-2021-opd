use regex::Regex;
use std::fmt;
use std::fs::read_to_string;

fn read_text(filename: &str) -> String {
    read_to_string(filename).unwrap()
}

#[derive(Debug)]
struct Cell {
    value: u32,
    marked: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = if self.marked { "M" } else { " " };
        write!(f, "{}{}", self.value.to_string(), m)
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Cell>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            for cell in row.iter() {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Board {
    pub fn from_input(lines: Vec<String>) -> Board {
        let board = lines
            .iter()
            .filter(|line| line.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|x| Cell {
                        value: x.parse().unwrap(),
                        marked: false,
                    })
                    .collect()
            })
            .collect::<Vec<_>>();

        Board { rows: board }
    }

    pub fn mark(&mut self, value: u32) {
        self.rows.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if cell.value == value {
                    cell.marked = true;
                }
            })
        })
    }

    pub fn wins(&self) -> bool {
        let wins_horiozntally = self
            .rows
            .iter()
            .filter(|row| row.iter().filter(|cell| cell.marked).count() == row.len())
            .count()
            > 0;

        let mut wins_vertically = false;
        self.rows.iter().for_each(|row| {
            let mut marked = 0;
            row.iter().for_each(|num| {
                if num.marked {
                    marked += 1;
                }
            });
            if marked == row.len() {
                wins_vertically = true;
            }
        });
        wins_horiozntally || wins_vertically
    }

    pub fn unmarked_values(&self) -> Vec<u32> {
        self.rows
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|cell| !cell.marked)
                    .map(|cell| cell.value)
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn solve_first(mut boards: Vec<Board>, draws: Vec<u32>) -> u32 {
    let mut winning_score: Option<u32> = None;
    'l: for draw in draws {
        for board in &mut boards {
            board.mark(draw);
            if board.wins() {
                winning_score = Some(board.unmarked_values().iter().sum::<u32>() * draw);
                break 'l;
            }
        }
    }

    winning_score.unwrap()
}

fn main() {
    let text = read_text("./input.txt");
    let pattern = Regex::new(r"\n\n").unwrap();
    let result: Vec<&str> = pattern.split(text.as_str()).collect();
    match result.as_slice() {
        [raw_draws, boards @ ..] => {
            let boards = boards
                .to_vec()
                .iter()
                .map(|board| Board::from_input(board.split('\n').map(|x| x.to_string()).collect()))
                .collect::<Vec<_>>();

            let draws = raw_draws
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let winning_score = solve_first(boards, draws);

            println!("{:?}", winning_score);

            // boards.iter().for_each(|board| println!("{}", board));
        }
        _ => println!("Huh?"),
    };
}
