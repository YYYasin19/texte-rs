use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::editor::Position;


pub struct Size {
    pub w: u16,
    pub h: u16,
}

pub struct Terminal {
    size: Size,
    // not pub, nobody can modify size
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                w: size.0,
                h: size.1.saturating_sub(2), // status bar is 2 lines
            },
            _stdout: stdout().into_raw_mode()?, // may panic
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    // static utility functions
    pub fn clear_screen() {
        Self::term_cmd(termion::clear::All);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(p: &Position) {
        // destructure
        let Position { x, y } = p;
        Self::term_cmd(
            termion::cursor::Goto(
                x.saturating_add(1) as u16,
                y.saturating_add(1) as u16,
            )
        )
    }

    pub fn set_cursor_visible(val: bool) {
        if val {
            Self::term_cmd(
                termion::cursor::Show
            )
        } else {
            Self::term_cmd(
                termion::cursor::Hide
            )
        }
    }

    pub fn clear_current_line() {
        Self::term_cmd(
            termion::clear::CurrentLine
        )
    }

    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<Key, io::Error> {
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

    pub fn set_bg_color(color: color::Rgb) {
        Self::term_cmd(color::Bg(color));
    }

    pub fn reset_bg_color() {
        Self::term_cmd(color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}