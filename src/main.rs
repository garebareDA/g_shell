use termion::*;
use termion::event::{Key, Event, MouseEvent};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Faild to read line");
        print!("{}", line);
    }
}
