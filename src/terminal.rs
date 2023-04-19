use crate::editor::Position;
use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size::new(size.0, size.1.saturating_sub(2)),
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All)
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine)
    }
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16))
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide)
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show)
    }
    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color))
    }
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset))
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color))
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset))
    }
}
