use termion::event::Key;
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

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let key = Terminal::read_key()?; // propagate the error above
        match key {
            Key::Ctrl('q') => self.will_quit = true,
            _ => ()
        }

        Ok(()) // return empty result for now
    }

    /// called for every input stroke; cleans stdout and writes a complete screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);

        if self.will_quit {
            println!("see you soon :)");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        return Terminal::flush();
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().h {
            println!("#\r");
        }
    }
}


/// clear the screen and write the error afterwards
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}