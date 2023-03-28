use termion::event::Key;
use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BORDER_CHAR: char = '🚀';


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

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("texte-rs -- v{}", VERSION);
        let width = self.terminal.size().w as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("{}{}{}", BORDER_CHAR, spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    /// called for every input stroke; cleans stdout and writes a complete screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::set_cursor_visible(false);
        Terminal::cursor_position(0, 0);

        if self.will_quit {
            Terminal::clear_screen();
            println!("see you soon :)");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::set_cursor_visible(true);
        return Terminal::flush();
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().h;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else { println!("{}\r", BORDER_CHAR); }

            // println!("{}{}{}\r", "#", " ".repeat((self.terminal.size().w - 2) as usize), "#")
        }
    }
}


/// clear the screen and write the error afterwards
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}