mod structs;

use pancurses::*;
use structs::ui::UI;
use structs::credentials::*;
fn main() {
    let window = initscr();
    let mut ui = UI::new(window);
    
    let mut credentials_arr: Vec<Credential> = Vec::new();
    
    loop {
        ui.init(&mut credentials_arr);
        let user_input = ui.await_user_input();

        match user_input {
            Input::Character('q') => {
                break;
            }
            Input::Character('i') => {
                ui.create_credential_mode();
            }
            _ => {}
        }
    }

    endwin();
}
