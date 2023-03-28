use termion::event::Key;
use crate::file::File;
use crate::Terminal;
use crate::Row;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BORDER_CHAR: char = '🚀';

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    will_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    file: File,
}

impl Editor {
    pub fn default() -> Self {

        // open a default file
        let args: Vec<String> = std::env::args().collect();
        let file = if args.len() > 1 {
            let file_path = &args[1];
            File::open(file_path).unwrap_or_default()
        } else {
            File::default()
        };


        Self {
            will_quit: false,
            terminal: Terminal::default().expect("Terminal should be initialized"),
            cursor_position: Position::default(),
            file,
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
            Key::Up | Key::Down | Key::Left | Key::Right
            | Key::PageDown | Key::PageUp | Key::Home | Key::End
            => self.move_cursor(key),
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

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().w as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    /// called for every input stroke; cleans stdout and writes a complete screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::set_cursor_visible(false);

        // seems unimportant but this determines where we start writing/drawing
        Terminal::cursor_position(&Position::default());

        if self.will_quit {
            Terminal::clear_screen();
            println!("see you soon :)");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::set_cursor_visible(true);
        return Terminal::flush();
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().h;
        for display_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.file.row(display_row as usize) {
                self.draw_row(row);
            } else if self.file.is_empty() {
                if display_row == height / 2 {
                    self.draw_welcome_message();
                } else {
                    println!("{}\r", BORDER_CHAR);
                }
            }

            // println!("{}{}{}\r", "#", " ".repeat((self.terminal.size().w - 2) as usize), "#")
        }
    }

    /// changes the cursor position, which, when redrawn, appears at the correct place on the screen
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.h.saturating_sub(1) as usize;
        let width = size.w.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => { if y < height { y = y.saturating_add(1) } }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => { if x < width { x = x.saturating_add(1) } }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }
}


/// clear the screen and write the error afterwards
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}