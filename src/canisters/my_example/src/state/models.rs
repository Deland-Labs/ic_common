use std::collections::HashSet;

use candid::{CandidType, Deserialize, Principal};

use crate::models::User;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub(crate) struct UserStable {
    id: Principal,
    name: String,
    email: String,
}

// &User -> UserStable
impl From<&User> for UserStable {
    fn from(user: &User) -> Self {
        UserStable {
            id: user.get_id().clone(),
            name: user.get_name().clone(),
            email: user.get_email().clone(),
        }
    }
}

// &UserStable -> User
impl From<&UserStable> for User {
    fn from(user: &UserStable) -> Self {
        User::new(
            user.id.clone(),
            user.name.clone(),
            user.email.clone(),
        )
    }
}
