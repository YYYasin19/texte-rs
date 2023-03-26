use std::io::{self, stdout};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

fn die(e: io::Error) {
    panic!("{}", e)
}

fn main() {

    // bind this to a variable s.t. the terminal does not exit raw mode and go into cooked mode
    // using _ as a prefix to a variable name is a convention to indicate that the variable is unused
    let _stdout = stdout().into_raw_mode().unwrap(); // ignore the error here
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key { // shadowing
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key)
            }
            Err(e) => die(e)
        }
    }
}
