use std::io::{self, Stdout, Write};

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{self},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

pub struct Tiles {
    cols: u16,
    rows: u16,
    canvas: Vec<char>,
    rng: ThreadRng,
    choices: Vec<char>,
    stdout: Stdout,
}

impl Tiles {
    pub fn new(choices: Vec<char>) -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;
        Ok(Self {
            cols: 0,
            rows: 0,
            canvas: Vec::new(),
            rng: thread_rng(),
            choices,
            stdout,
        })
    }

    pub fn randomize(&mut self) -> Result<(), io::Error> {
        let (cols, rows) = size()?;
        if self.cols != cols || self.rows != rows {
            self.cols = cols;
            self.rows = rows;

            let canvas_size = self.rows as usize * self.cols as usize;
            self.canvas = Vec::with_capacity(canvas_size);
            self.canvas.resize(canvas_size, '\0');

            for y in 0..self.rows {
                for x in 0..self.cols {
                    let c = *self
                        .choices
                        .choose(&mut self.rng)
                        .expect("There must be at least 2 choices.");
                    self.set_char(x, y, c)?;
                }
            }
        } else {
            for _ in 0..std::cmp::max(self.rows, self.cols) {
                let x = self.rng.gen_range(0..self.cols);
                let y = self.rng.gen_range(0..self.rows);
                let cur_char = self.get_char(x, y);
                let choices = self
                    .choices
                    .iter()
                    .copied()
                    .filter(|&c| c != cur_char)
                    .collect::<Vec<_>>();
                let new_char = choices
                    .choose(&mut self.rng)
                    .expect("There must be at least 2 choices.");

                self.set_char(x, y, *new_char)?;
            }
        }
        self.stdout.flush()
    }

    fn get_char(&self, x: u16, y: u16) -> char {
        self.canvas[y as usize * self.cols as usize + x as usize]
    }

    fn set_char(&mut self, x: u16, y: u16, c: char) -> Result<(), io::Error> {
        self.canvas[y as usize * self.cols as usize + x as usize] = c;
        queue!(self.stdout, cursor::MoveTo(x, y), style::Print(c))?;
        Ok(())
    }
}

impl Drop for Tiles {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.stdout, LeaveAlternateScreen, cursor::Show).unwrap();
    }
}
