use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;


pub struct Editor {
    will_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { will_quit: false }
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

    fn refresh_screen(&self) -> Result<(), io::Error> {
        // print!("\x1b[2J"); // escape sequence
        // clear screen and move cursor to start
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        if self.will_quit {
            println!("see you soon :)");
        }

        stdout().flush() // could return an error
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

/// clear the screen and write the error afterwards
fn die(e: io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}