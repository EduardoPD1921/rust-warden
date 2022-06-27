use pancurses::*;
use std::path::Path;
use std::fs::{self, File};
fn main() {
    let window = initscr();
    window.keypad(true);
    noecho();

    let mut credentials_arr: Vec<(String, String, String)> = Vec::new();

    let is_credentials_file_created = Path::new("credentials.txt").exists();
    if is_credentials_file_created {
        let file_buf = fs::read_to_string("credentials.txt").unwrap();

        for line in file_buf.lines() {
            let mut chunk = line.split(';');

            let name = chunk.next().unwrap().to_string();
            let user = chunk.next().unwrap().to_string();
            let password = chunk.next().unwrap().to_string();

            credentials_arr.push((name, user, password));
        }
    }

    for (index, item) in credentials_arr.iter().enumerate() {
        window.mvaddstr(0, 0, "Vault:");
        window.mvaddstr((index + 2) as i32, 0, &item.0);
    }
    
    loop {
        let test = window.getch().unwrap();

        match test {
            Input::Character('\n') => {
                break;
            },
            _ => {}
        }
    }

    endwin();
}
