use pancurses::{noecho, Window, Input};
use std::fs::{self};
use std::path::Path;

pub struct UI {
	window: Window
}

impl UI {
	pub fn new(window: Window) -> Self {
		Self { window }
	}

	pub fn init(&self, credentials_arr: &mut Vec<(String, String, String)>) {
		self.window.keypad(true);
		noecho();

		self.draw_vaults_menu(credentials_arr);
	}

	fn draw_vaults_menu(&self, credentials_arr: &mut Vec<(String, String, String)>) {
		let is_credentials_file_created = Path::new("credentials.txt").exists();
		if is_credentials_file_created {
			let file_buf = fs::read_to_string("credentials.txt").unwrap();

			for line in file_buf.lines() {
				let mut chunk = line.split(';');

				let vault_name = chunk.next().unwrap().to_string();
				let vault_user = chunk.next().unwrap().to_string();
				let vault_password = chunk.next().unwrap().to_string();

				credentials_arr.push((vault_name, vault_user, vault_password));
			}
		}

		self.window.mvaddstr(0, 0, "Vault:");

		if credentials_arr.is_empty() {
			self.window.mvaddstr(2, 0, "None credentials have been found.");
		} else {
			for (index, item) in credentials_arr.iter().enumerate() {
				self.window.mvaddstr((index + 2) as i32, 0, &item.0);
			}
		}
	}

	pub fn await_user_input(&self) -> Input {
		let user_input = self.window.getch().unwrap();
		user_input
	}
}