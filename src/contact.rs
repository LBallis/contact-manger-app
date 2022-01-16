use serde::Deserialize;
#[derive(Debug,Deserialize)]
pub struct Contact {
    id: i32,
    name: String,
    email: String
}

impl Contact {
    pub fn new(id: i32, name: String, email: String) -> Self {
        Self{id, name, email}
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn set_name(&mut self, name: &str){
        self.name = name.to_owned();
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_owned()
    }
}