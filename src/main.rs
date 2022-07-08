mod structs;

use pancurses::*;
use structs::ui::UI;
fn main() {
    let mut ui = init_app();
    
    loop {
        ui.init();
        let user_input = ui.await_user_input();

        match user_input {
            Input::Character('w') => {
                ui.move_cursor_up();
            }
            Input::Character('s') => {
                ui.move_cursor_down();
            }
            Input::Character('i') => {
                ui.create_credential_mode();
            }
            Input::Character('\n') => {
                ui.show_selected_credential();
            }
            Input::Character('q') => {
                break;
            }
            _ => {}
        }
    }

    endwin();
}

fn init_app() -> UI {
    let window = initscr();
    let ui = UI::new(window);
    ui
}
