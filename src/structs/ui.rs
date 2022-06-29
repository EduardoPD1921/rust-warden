use pancurses::{noecho, Window, Input};
use std::fs::{self};
use std::path::Path;

use super::credentials::Credential;

pub struct UI {
	window: Window
}

impl UI {
	pub fn new(window: Window) -> Self {
		Self { window }
	}

	pub fn init(&self, credentials_arr: &mut Vec<Credential>) {
        self.window.clear();
		self.window.keypad(true);
		noecho();

		self.draw_vaults_menu(credentials_arr);
	}

    pub fn insert_credential_name(&self) {
        let mut credential_name: Vec<char> = Vec::new();
        self.window.clear();

        self.window.mvaddstr(0, 0, "Insert your credential name:");
        self.window.mv(1, 0);

        loop {
            let user_input = self.await_user_input();

            match user_input {
                Input::Character('\n') => {
                    break;
                }
                Input::Character(c) => {
                    self.window.addch(c);
                    credential_name.push(c);
                }
                Input::KeyBackspace => {
                    credential_name.pop();
                    self.window.mv(self.window.get_cur_y(), self.window.get_cur_x() - 1);
                    self.window.delch();
                }
                _ => {}
            }
        }
    }
    
    pub fn await_user_input(&self) -> Input {
		let user_input = self.window.getch().unwrap();
		user_input
	}

    fn move_cursor_max(&self) {
        self.window.mv(self.window.get_max_y() - 1, self.window.get_max_x() - 1);
    }

	fn draw_vaults_menu(&self, credentials_arr: &mut Vec<Credential>) {
		let is_credentials_file_created = Path::new("credentials.txt").exists();
		if is_credentials_file_created {
			let file_buf = fs::read_to_string("credentials.txt").unwrap();

			for line in file_buf.lines() {
				let mut chunk = line.split(';');

				let credential_name = chunk.next().unwrap().to_string();
				let credential_user = chunk.next().unwrap().to_string();
				let credential_password = chunk.next().unwrap().to_string();

                let credential = Credential::new(credential_name, credential_user, credential_password);
                credentials_arr.push(credential);
			}
		}

		self.window.mvaddstr(0, 0, "Vault:");

		if credentials_arr.is_empty() {
			self.window.mvaddstr(1, 0, "None credentials have been found.");
		} else {
			for (index, item) in credentials_arr.iter().enumerate() {
				self.window.mvaddstr((index + 1) as i32, 0, &item.name);
			}
		}

        self.draw_controls();
        self.move_cursor_max();
	}

    fn draw_controls(&self) {
        let max_y_terminal = self.window.get_max_y() - 1;

        self.window.mvaddstr(max_y_terminal, 0, "[w a s d: navigation]");
        self.window.mvaddstr(max_y_terminal, 22, "[i: insert new credential]");
        self.window.mvaddstr(max_y_terminal, 49, "[q: exit]");
    }
}