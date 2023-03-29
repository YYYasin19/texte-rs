use std::cmp;
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
    file_offset: Position, // needed for scrolling
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
            file_offset: Position::default(),
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
        self.scroll(); // scroll at every key press
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

        // seems unimportant but this determines where we start writing/drawing
        Terminal::cursor_position(&Position::default());

        if self.will_quit {
            Terminal::clear_screen();
            println!("see you soon :)");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.file_offset.x),
                y: self.cursor_position.y.saturating_sub(self.file_offset.y),
            });
        }
        Terminal::set_cursor_visible(true);
        return Terminal::flush();
    }


    /// return the part of the file that represents the string we want to display on the screen
    /// example: the user has scrolled to the right, so we want to display the file starting at the
    /// current file offset
    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().w as usize;
        let start = self.file_offset.x; // start at offset instead of 0
        let end = start + width;

        // display only the part of the row that fits on the screen
        println!("{}\r", row.render(start, end));
    }

    /// draw relevant parts of file on the screen
    fn draw_rows(&self) {
        let height = self.terminal.size().h;
        for display_row in 0..height - 1 {
            Terminal::clear_current_line();

            // try to get the file row at the current display row + the file offset
            if let Some(row) = self.file.row(display_row as usize + self.file_offset.y) {
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

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().w as usize;
        let height = self.terminal.size().h as usize;
        let mut file_offset = &mut self.file_offset;

        if y < file_offset.y {
            // we have scrolled up, so the cursor is above the screen
            file_offset.y = y;
        } else if y > file_offset.y.saturating_add(height) { // the cursor is below the screen
            // change offset such that the last line of the file is at the bottom of the screen
            file_offset.y = y.saturating_sub(height).saturating_add(1);
        }

        // equivalent logic for x direction
        if x < file_offset.x {
            file_offset.x = x;
        } else if x > file_offset.x.saturating_add(width) {
            file_offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    /// changes the cursor position, which, when redrawn, appears at the correct place on the screen
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.file.len() as usize; // max height is file len
        let width = if let Some(row) = self.file.row(y) {
            row.len()
        } else {
            0
        };


        // TODO: when changing rows (up or down) cursor should be at max x position of the new row
        match key {
            Key::Up => {
                y = y.saturating_sub(1);
            }
            Key::Down => {
                if y < height { y = y.saturating_add(1) };
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => { if x < width { x = x.saturating_add(1) } }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        // this is now the width of the next row
        let width = if let Some(row) = self.file.row(y) {
            row.len()
        } else {
            0
        };
        x = cmp::min(x, width);
        self.cursor_position = Position { x, y }
    }
}


/// clear the screen and write the error afterwards
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}