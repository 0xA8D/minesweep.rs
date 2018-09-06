// Terminal Minesweeper
extern crate rand;

use rand::{thread_rng, Rng};
use std::fmt;

const MIN_DIMENSION: isize = 8;
const MAX_DIMENSION: isize = 16;
const BOMB_PROBABILITY: f64 = 1.0 / 10.0;

pub struct Cell {
    is_flag: bool,
    is_visible: bool,
    is_mine: bool,
    neighbors: usize,
}

impl Cell {
    fn new<R: Rng>(rng: &mut R) -> Cell {
        Cell {
            is_mine: rng.gen_bool(BOMB_PROBABILITY),
            is_visible: false,
            is_flag: false,
            neighbors: 0,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut c = '.';
        if self.is_visible {
            if self.is_mine {
                c = 'x';
            } else {
                c = match self.neighbors {
                    0 => ' ',
                    n @ _ => format!("{}",n).chars().next().unwrap(),
                };
            }
        } else if self.is_flag {
            c = 'F';
        }
        write!(f, "{}", c)
    }
}

pub struct Board {
    cells: Vec<Vec<Cell>>, // [row][col]
}

impl Board {
    /// Initialize new random board
    pub fn new() -> Board {
        let mut rng = thread_rng();
        let x = rng.gen_range(MIN_DIMENSION, MAX_DIMENSION);
        let y = rng.gen_range(MIN_DIMENSION, MAX_DIMENSION);

        // Intialize cells
        let mut cells = vec![];
        for _ in 0..y {
            let mut row = vec![];
            for _ in 0..x {
                row.push(Cell::new(&mut rng));
            }
            cells.push(row);
        }

        let mut board = Board { cells };

        // Count neighboring mines
        for row in 0..y {
            for col in 0..x {
                // Certain to be a cell, so unwrap()
                if board.getcell(row, col).unwrap().is_mine {
                    for r in (row - 1)..=(row + 1) {
                        for c in (col - 1)..=(col + 1) {
                            if let Some(c) = board.getcell(r, c) {
                                c.neighbors += 1;
                            }
                        }
                    }
                }
            }
        }

        board
    }

    /// Safe reader/writer if cell exists, returns &mut of cell
    pub fn getcell(&mut self, row: isize, col: isize) -> Option<&mut Cell> {
        if row >= 0
            && col >= 0
            && (row as usize) < self.cells.len()
            && (col as usize) < self.cells[0].len()
        {
            Some(&mut self.cells[row as usize][col as usize])
        } else {
            None
        }
    }

    /// Player toggles cell's flag
    pub fn toggle_flag(&mut self, row: isize, col: isize) {
        if let Some(c) = self.getcell(row, col) {
            if c.is_visible {
                c.is_flag = false;
            } else {
                c.is_flag = !c.is_flag;
            }
        }
    }

    /// Player probes cell, Ok() game continues, Err() game ends
    pub fn probe(&mut self, row: isize, col: isize) -> Result<(), ()> {
        let mut recurse = false;
        if let Some(c) = self.getcell(row, col) {
            if c.is_mine && return Err(()) {}

            if !c.is_visible {
                c.is_visible = true;
                c.is_flag = false;
                if c.neighbors == 0 {
                    recurse = true;
                }
            }
        }
        // Recurse flag used because we must drop &mut Cell from above
        // If no neighboring bombs, probe all neighbors
        if recurse {
            for r in (row - 1)..=(row + 1) {
                for c in (col - 1)..=(col + 1) {
                    self.probe(r, c).unwrap();
                }
            }
        }
        Ok(())
    }

    // Board is won if all flags are mines, and all mines are flags.
    pub fn is_won(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if cell.is_mine != cell.is_flag {
                    return false
                }
            }
        }
        return true
    }

    // Uncover all cells after win/lose
    pub fn make_visible(&mut self) {
        for row in &mut self.cells {
            for mut cell in row {
                cell.is_visible = true;
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Column numbers, tens digit
        write!(f, "   ");
        for x in 1..=(self.cells[0].len()) {
            match x {
                0...9 => write!(f, " "),
                _ => write!(f, "{}", x / 10),
            }?;
        }
        // Column numbers, ones digit
        write!(f, "\n   ")?;
        (1..=(self.cells[0].len())).map(|col| write!(f, "{}", col % 10)).count();
        write!(f, "\n\n")?;

        for (mut i, ref row) in self.cells.iter().enumerate() {
            i += 1;
            // Row numbers
            match i {
                0...9 => write!(f, " {} ", i),
                _ => write!(f, "{} ", i),
            }?;

            // Display cell
            row.iter().map(|cell| write!(f, "{}", cell)).count();
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// Read player input: row col <flag>
fn parse_input(input: &mut String) -> Result<(isize, isize, bool), ()> {
    let input = input.split_whitespace().collect::<Vec<&str>>();

    if input.len() != 2 && input.len() != 3 && return Err(()) {}

    if let Ok(row) = input[0].parse() {
        if let Ok(col) = input[1].parse() {
            return Ok((row, col, input.len() == 3));
        }
    }
    return Err(());
}

fn main() {
    println!("\nTerminal Minesweeper!\nDo you feel lucky? Well, do ya, cypherpunk?\n");

    let mut board = Board::new();

    loop {
        println!("{}", board);
        println!("Enter \"row col\" to probe, or \"row col F\" to toggle flag.\n"
        );

        use std::io;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok((mut row, mut col, flag)) = parse_input(&mut input) {
            row -= 1;
            col -= 1;
            if flag {
                board.toggle_flag(row, col);
            } else if let Err(_) = board.probe(row, col) {
                // End game
                println!("\n!!!BOOOOOOOOOM!!! GAME OVER!\n");
                board.make_visible();
                println!("{}", board);
                break;
            }
            if board.is_won() {
                // Won game
                println!("\nYOU WON THE GAME!\n");
                board.make_visible();
                println!("{}", board);
                break;
            }
        } else {
            println!("\n Invalid input, try again.\n\n")
        }
    }
}
