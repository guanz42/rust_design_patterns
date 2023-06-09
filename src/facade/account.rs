#![allow(dead_code)]

pub struct Account {
    name: String,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn check(&self, name: &String) -> Result<(), String> {
        if &self.name != name {
            return Err("Account name is error".into());
        }

        println!("Accoutn verified");
        Ok(())
    }
}
