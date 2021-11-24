use std::fmt::{Display, Formatter};

use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct User {
    id: Principal,
    name: String,
    email: String,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ id: {}, name: {}, email: {} }}", self.id, self.name, self.email)
    }
}


impl User {
    pub fn new(id: Principal, name: String, email: String) -> Self {
        User {
            id,
            name,
            email,
        }
    }

    pub fn get_id(&self) -> &Principal {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }
}
