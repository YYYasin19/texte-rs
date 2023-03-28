use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use crate::Terminal;


pub struct Editor {
    will_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            will_quit: false,
            terminal: Terminal::default().expect("Terminal should be initialized"),
        }
    }

    /// takes control over stdin/out in raw mode and processes keys
    pub fn run(&mut self) {

        // infinite loop to process key presses until an Error (e.g. Ctrl-Q) comes up
        loop {

            // this will be run _before_ exiting
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.will_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let key = read_key()?; // propagate the error above
        match key {
            Key::Ctrl('q') => self.will_quit = true,
            _ => ()
        }

        Ok(()) // return empty result for now
    }

    /// called for every input stroke; cleans stdout and writes a complete screen
    fn refresh_screen(&self) -> Result<(), io::Error> {
        // print!("\x1b[2J"); // escape sequence
        // clear screen and move cursor to start
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        if self.will_quit {
            println!("see you soon :)");
        } else {
            self.draw_rows();
            term_cmd(termion::cursor::Goto(1, 1))
        }

        stdout().flush() // could return an error
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().h {
            println!("~\r");
        }
    }
}

// utility function
fn read_key() -> Result<Key, io::Error> {
    loop {
        // loop until stdio returns a key (Option - can be None or Some)
        if let Some(k) = io::stdin().lock().keys().next() {
            return k;
        }
    }
}

fn term_cmd<T: std::fmt::Display>(cmd_seq: T) {
    print!("{}", cmd_seq)
}

/// clear the screen and write the error afterwards
fn die(e: io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}