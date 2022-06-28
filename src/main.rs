mod structs;

use pancurses::*;
use structs::ui::UI;
fn main() {
    let window = initscr();
    let ui = UI::new(window);
    
    let mut credentials_arr: Vec<(String, String, String)> = Vec::new();
    ui.init(&mut credentials_arr);
    
    loop {
        let user_input = ui.await_user_input();

        match user_input {
            Input::Character('q') => {
                break;
            },
            _ => {}
        }
    }

    endwin();
}
