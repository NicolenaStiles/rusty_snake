extern crate termion;

use std::{thread, time};

use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let one_second = time::Duration::from_millis(1000);
    let mut now = time::Instant::now();
    let mut then = now + one_second;
    println!("BEGIN!!");

    loop {
        thread::sleep(one_second);
        println!("hemlo");
    }
}
