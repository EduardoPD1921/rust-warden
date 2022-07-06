use std::fs::{File, self};
use std::io::{LineWriter, Write};
use std::path::Path;

pub struct Credential {
    pub name: String,
    user: String,
    password: String
}

impl Credential {
    pub fn new(credential_name: String, credential_user: String, credential_password: String) -> Self {
        Self { name: credential_name, user: credential_user, password: credential_password }
    }

    pub fn save_to_file(self) {
		let is_credentials_file_created = Path::new("credentials.txt").exists();
        if is_credentials_file_created {
            self.save_to_existed_file();
        } else {
            self.create_file_and_save();
        }
    }

    fn save_to_existed_file(self) {
        let mut credentials_arr: Vec<Credential> = Vec::new();
        let file_buf = fs::read_to_string("credentials.txt").unwrap();

        for line in file_buf.lines() {
            let mut chunk = line.split(';');

            let name = chunk.next().unwrap();
            let user = chunk.next().unwrap();
            let password = chunk.next().unwrap();

            let credential = Self::new(name.to_owned(), user.to_owned(), password.to_owned());
            credentials_arr.push(credential);
        }

        credentials_arr.push(self);

        let file = File::create("credentials.txt").unwrap();
        let mut file = LineWriter::new(file);

        for credential in credentials_arr {
            let to_save_credential_str = format!("{};{};{}\n", credential.name, credential.user, credential.password);
            file.write_all(to_save_credential_str.as_bytes()).unwrap();
        }

        file.flush().unwrap();
    }

    fn create_file_and_save(self) {
        let file = File::create("credentials.txt").unwrap();
        let mut file = LineWriter::new(file);

        let credential = format!("{};{};{}\n", self.name, self.user, self.password);

        file.write_all(credential.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}