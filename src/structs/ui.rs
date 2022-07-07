use pancurses::{noecho, Window, Input, COLOR_PAIR, start_color, init_pair, COLOR_WHITE, COLOR_BLACK};
use std::fs;
use std::path::Path;

use rand::{thread_rng, Rng};

use super::credentials::Credential;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;
pub struct UI {
	window: Window,
    credential_name: Option<String>,
    credential_user: Option<String>,
    credential_password: Option<String>,
    cursor_y_index: i32,
    qtd_credentials: i32,
    show_password: bool
}

// TODO: 
// - put all loop interfaces into a separate file

impl UI {
	pub fn new(window: Window) -> Self {
		Self { window, 
            cursor_y_index: 0,
            qtd_credentials: 0, 
            credential_name: None, 
            credential_user: None, 
            credential_password: None, 
            show_password: false 
        }
	}

	pub fn init(&mut self) {
		self.window.keypad(true);
		noecho();

        start_color();
        init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
        init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_WHITE);

        self.qtd_credentials = 0;
        self.credential_name = None;
        self.credential_user = None;
        self.credential_password = None;

		self.draw_vaults_menu();
	}

    pub fn create_credential_mode(&mut self) {
        self.window.clear();

        self.insert_credential_parameter("name", 0);
        self.insert_credential_parameter("user", 1);
        self.insert_credential_password();

    }
    
    pub fn await_user_input(&self) -> Input {
		let user_input = self.window.getch().unwrap();
		user_input
	}

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y_index > 0 {
            self.cursor_y_index -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y_index < self.qtd_credentials - 1 {
            self.cursor_y_index += 1;
        }
    }

    fn move_cursor_max(&self) {
        self.window.mv(self.window.get_max_y() - 1, self.window.get_max_x() - 1);
    }

	fn draw_vaults_menu(&mut self) {
        self.window.clear();

        let mut credentials_arr: Vec<Credential> = Vec::new();

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
                self.qtd_credentials += 1;
			}
		}

		self.window.mvaddstr(0, 0, "Vault:");

		if credentials_arr.is_empty() {
			self.window.mvaddstr(1, 0, "None credentials have been found.");
		} else {
			for (index, item) in credentials_arr.iter().enumerate() {
                let pair_style = if self.cursor_y_index == index as i32 { HIGHLIGHTED_PAIR as u32 } else { REGULAR_PAIR as u32 };
    
                self.window.attron(COLOR_PAIR(pair_style));
                self.window.mvaddstr((index + 1) as i32, 0, &item.name);
                self.window.attroff(COLOR_PAIR(pair_style));
			}
		}

        self.draw_controls();
        self.move_cursor_max();
	}

    fn draw_controls(&self) {
        let max_y_terminal = self.window.get_max_y() - 1;

        self.window.mvaddstr(max_y_terminal, 0, "[w s: navigation]");
        self.window.mvaddstr(max_y_terminal, 18, "[i: insert new credential]");
        self.window.mvaddstr(max_y_terminal, 45, "[q: exit]");
    }

    fn insert_credential_parameter(&mut self, parameter: &str, y_pos: i32) {
        self.draw_inserted_parameters();
        
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

        self.draw_inserted_parameters();
    }

    fn insert_credential_password(&mut self) {
        self.window.mvaddstr(2, 0, "Do you want a generated password? [y] or [n]");

        loop {
            let user_input = self.await_user_input();

            match user_input {
                Input::Character('n') => {
                    self.insert_credential_parameter("password", 2);
                    break;
                }
                Input::Character('y') => {
                    self.generate_password();
                    break;
                }
                _ => {}
            }
        }
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

    fn draw_inserted_parameters(&mut self) {
        self.window.clear();

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
                let password = if self.show_password {
                    p.to_owned()
                } else {
                    let mut hidden_password_arr: Vec<char> = Vec::new();
                    for _c in p.chars() {
                        hidden_password_arr.push('*');
                    }

                    hidden_password_arr.iter().collect()
                };

                let inserted_password = format!("Credential password: {}", password);
                self.window.mvaddstr(text_y, 0, inserted_password);
                self.draw_finish_screen_controls();

                loop {
                    let user_input = self.await_user_input();

                    match user_input {
                        Input::Character('\n') => {
                            let owned_name = self.credential_name.as_ref().unwrap().to_owned();
                            let owned_user = self.credential_user.as_ref().unwrap().to_owned();
                            let owned_password = self.credential_password.as_ref().unwrap().to_owned();

                            let credential = Credential::new(owned_name, owned_user, owned_password);
                            credential.save_to_file();

                            break;
                        }
                        Input::Character('t') => {
                            self.show_password = !self.show_password;
                            self.draw_inserted_parameters();
                            break;
                        }
                        _ => {}
                    }
                }
            }
            None => {}
        }
    }

    fn draw_finish_screen_controls(&self) {
        self.window.mvaddstr(self.window.get_max_y() - 1, 0, "[Enter: save credential]");
        self.window.mvaddstr(self.window.get_max_y() - 1, 25, "[t: toggle password visibility]");

        self.move_cursor_max();
    }

    fn generate_password(&mut self) {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 15;
        let mut rng = thread_rng();

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        self.credential_password = Some(password);

        self.window.clear();
        self.draw_inserted_parameters();
    }
}