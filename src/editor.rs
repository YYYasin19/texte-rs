use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


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

            // if let is like a "match" but for just one specific case
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.will_quit {
                break;
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

fn die(e: io::Error) {
    panic!("{}", e);
}