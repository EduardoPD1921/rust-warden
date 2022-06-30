use pancurses::{noecho, Window, Input};
use std::fs::{self};
use std::path::Path;

use super::credentials::Credential;
pub struct UI {
	window: Window,
    credential_name: Option<String>,
    credential_user: Option<String>,
    credential_password: Option<String>
}


impl UI {
	pub fn new(window: Window) -> Self {
		Self { window, credential_name: None, credential_user: None, credential_password: None }
	}

	pub fn init(&self, credentials_arr: &mut Vec<Credential>) {
        self.window.clear();
		self.window.keypad(true);
		noecho();

		self.draw_vaults_menu(credentials_arr);
	}

    pub fn create_credential_mode(&mut self) {
        self.window.clear();

        self.insert_credential_parameter("name", 0);
        self.insert_credential_parameter("user", 1);
        self.insert_credential_parameter("password", 2);
        self.window.getch();
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

    fn insert_credential_parameter(&mut self, parameter: &str, y_pos: i32) {
        let mut credential_parameter_arr: Vec<char> = Vec::new();
        let action_desc = format!("Insert your credential {}:", parameter);

        self.window.mvaddstr(y_pos, 0, action_desc);
        self.window.mv(y_pos + 1, 0);

        let credential_parameter = self.get_user_input_data(&mut credential_parameter_arr, parameter);

        match parameter {
            "name" => {
                self.credential_name = Some(credential_parameter);
            }
            "user" => {
                self.credential_user = Some(credential_parameter);
            }
            "password" => {
                self.credential_password = Some(credential_parameter);
            }
            _ => {}
        }

        self.window.clear();
        self.draw_inserted_parameters();
    }

    fn get_user_input_data(&self, arr: &mut Vec<char>, parameter_name: &str) -> String {
        loop {
            let user_input = self.await_user_input();

            match user_input {
                Input::Character('\n') => {
                    break;
                }
                Input::Character(c) => {
                    if parameter_name == "password" {
                        self.window.addch('*');
                    } else {
                        self.window.addch(c);
                    }
                    arr.push(c);
                }
                Input::KeyBackspace => {
                    arr.pop();
                    self.window.mv(self.window.get_cur_y(), self.window.get_cur_x() - 1);
                    self.window.delch();
                }
                _ => {}
            }
        }

        arr.iter().collect()
    }

    fn draw_inserted_parameters(&self) {
        let mut text_y = 0;

        match &self.credential_name {
            Some(n) => {
                let inserted_name = format!("Credential name: {}", n);
                self.window.mvaddstr(text_y, 0, inserted_name);
                text_y += 1;
            }
            None => {}
        }

        match &self.credential_user {
            Some(u) => {
                let inserted_user = format!("Credential user: {}", u);
                self.window.mvaddstr(text_y, 0, inserted_user);
                text_y += 1;
            }
            None => {}
        }

        match &self.credential_password {
            Some(p) => {
                let inserted_password = format!("Credential password: {}", p);
                self.window.mvaddstr(text_y, 0, inserted_password);
            }
            None => {}
        }
    }
}