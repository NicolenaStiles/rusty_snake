extern crate termion;

use std::{thread, time};

use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    // current position of snake head
    let mut current_position : (u16,u16) = (40,12);
    let one_second = time::Duration::from_millis(1000);
    let mut now = time::Instant::now();
    let mut then = now + one_second;

    // input and output declarations
    let mut stdout = stdout().into_raw_mode().unwrap();

    // writing the menu
    write!(stdout,
           "{}{}{}\u{2588}",
           termion::cursor::Goto(current_position.0, current_position.1),
           termion::clear::All,
           termion::cursor::Hide)
            .unwrap();

    stdout.flush().unwrap();

    let stdin = stdin();
    // thread::sleep(one_second);

    write!(stdout,
           "{}{}{}\u{2588}",
           termion::cursor::Goto(current_position.0,current_position.1),
           termion::clear::All,
           termion::cursor::Hide)
            .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                write!(stdout, "{}{}",termion::clear::All, termion::cursor::Show).unwrap();
                break;
            },
            Key::Up => {
                current_position.1 -= 1;
                write!(stdout,
                        "{}{}{}\u{2588}",
                        termion::clear::All,
                        termion::cursor::Goto(current_position.0,current_position.1),
                        termion::cursor::Hide)
                        .unwrap();
            }
            Key::Down => {
                current_position.1 += 1;
                write!(stdout,
                        "{}{}{}\u{2588}",
                        termion::clear::All,
                        termion::cursor::Goto(current_position.0,current_position.1),
                        termion::cursor::Hide)
                        .unwrap();
            }
            Key::Left => {
                current_position.0 -= 2;
                write!(stdout,
                        "{}{}{}\u{2588}",
                        termion::clear::All,
                        termion::cursor::Goto(current_position.0,current_position.1),
                        termion::cursor::Hide)
                        .unwrap();
            }
            Key::Right => {
                current_position.0 += 2;
                write!(stdout,
                        "{}{}{}\u{2588}",
                        termion::clear::All,
                        termion::cursor::Goto(current_position.0,current_position.1),
                        termion::cursor::Hide)
                        .unwrap();
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    stdout.flush().unwrap();

}
