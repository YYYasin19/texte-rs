use std::cmp;
use std::time::{Duration, Instant};
use termion::event::Key;
use crate::file::File;
use crate::Terminal;
use crate::Row;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BORDER_CHAR: char = '🚀';
// const STATUS_BAR_BG_COLOR: color::Rgb = color::Rgb(255, 255, 255);
// const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

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
    file_offset: Position,
    // needed for scrolling
    status_message: StatusMessage,
}

impl Editor {
    pub fn default() -> Self {

        // open a default file
        let args: Vec<String> = std::env::args().collect();
        let mut initial_status = String::from("Welcome! Press Ctrl-Q to quit.");
        let file = if args.len() > 1 {
            let file_path = &args[1];
            let file = File::open(file_path);
            if file.is_ok() {
                initial_status = String::from("file opened");
                file.unwrap()
            } else {
                initial_status = format!("error: could not open file: {}", file_path);
                File::default()
            }
        } else {
            File::default()
        };


        Self {
            will_quit: false,
            terminal: Terminal::default().expect("Terminal should be initialized"),
            cursor_position: Position::default(),
            file_offset: Position::default(),
            file,
            status_message: StatusMessage::from(initial_status),
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
            Key::Delete => self.file.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.file.delete(&self.cursor_position);
                }
            }
            Key::Char(c) => {
                self.file.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
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
            self.draw_rows(); // draw the file contents (len: file_len)
            self.draw_status_bar(); //  draw the status bar (len: 1)
            self.draw_message_bar(); // draw the message bar (len: 1)
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

        // status bar is the first two rows
        // example: real height: 1000, height: 1000-2 = 998 -> scroll from 2 to 1000
        for display_row in 0..height {
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

    /// depending on the cursor-position updates the file offset,
    /// s.t. we show the correct portion of the file.
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().w as usize;
        // subtract 1 from the terminal-height to accommodate for the status bar
        let height = (self.terminal.size().h - 1) as usize;
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
        let terminal_height = self.terminal.size().h as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let file_len = self.file.len() as usize; // max height is file len
        let mut width = if let Some(row) = self.file.row(y) {
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
                if y < file_len { y = y.saturating_add(1) };
            }
            Key::Left => {
                if x > 0 { // are we at the beginning of the row?
                    x = x.saturating_sub(1)
                } else if y > 0 { // are we at the beginning of the file?
                    y -= 1;
                    if let Some(row) = self.file.row(y) {
                        x = row.len(); // x at end of row
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1
                } else if y < file_len { // x at the end of the row, y not at the end of the file
                    // go to beginning of the next row
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y.saturating_add(terminal_height) < file_len {
                    y + terminal_height as usize
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y > terminal_height { y - terminal_height } else { 0 }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        // this is now the width of the next row
        width = if let Some(row) = self.file.row(y) {
            row.len()
        } else {
            0
        };
        x = cmp::min(x, width);
        self.cursor_position = Position { x, y }
    }

    /// Called on the correct line to print the status bar
    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().w as usize;

        let mut file_name = String::from("");
        if let Some(name) = &self.file.file_name {
            file_name = name.clone();
            file_name.truncate(30);
        }
        status = format!("{} - {} lines", file_name, self.file.len());

        let line_indicator = format!("{} / {}", self.cursor_position.y + 1, self.file.len());
        let len = status.len() + line_indicator.len();
        if width > len { // add spaces to fill the line to the right
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        // status.truncate(width);
        // macOS issue: https://github.com/pflenker/hecto-tutorial/issues/3 (fix in termion?)
        Terminal::set_status_bar_color();
        println!("{}\r", status);
        Terminal::reset_status_bar_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let msg = &self.status_message;
        // TODO: setup a message queue instead of just displaying the most current message
        // display the message only if it is less than 5 seconds old
        if (Instant::now() - msg.time) < Duration::new(5, 0) {
            println!(
                "{}\r",
                &msg.text[0..cmp::min(msg.text.len(), self.terminal.size().w as usize)]
            );
        }
    }
}


/// clear the screen and write the error afterwards
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

struct StatusMessage {
    text: String,
    time: Instant,
}


impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}