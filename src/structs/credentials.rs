// use std::fs::{self, File};
// use std::io::LineWriter;
// use std::path::Path;

pub struct Credential {
    pub name: String,
    user: String,
    password: String
}

impl Credential {
    pub fn new(credential_name: String, credential_user: String, credential_password: String) -> Self {
        Self { name: credential_name, user: credential_user, password: credential_password }
    }
}