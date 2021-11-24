use std::collections::HashMap;
use std::sync::Arc;

use candid::Principal;
use log::{debug, info};

use ic_common::errors::ServiceResult;

use crate::errors::{UserServiceError, UserServiceResult};
use crate::models::*;
use crate::state::USERS;

#[cfg(test)]
mod tests;

pub struct UserService {}

impl UserService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn get_user_by_id(&self, id: &Principal) -> UserServiceResult<User> {
        USERS.with(|users| {
            let users = users.borrow();
            let option = users.get(id).cloned();
            match option {
                Some(user) => Ok(user),
                None => Err(UserServiceError::UserNotFound),
            }
        })
    }

    // add new user with given id, name and email
    // if user exists, return error
    pub(crate) fn add_user(&self, id: &Principal, name: &str, email: &str) -> UserServiceResult<()> {
        USERS.with(|users| {
            let mut users = users.borrow_mut();
            if users.contains_key(id) {
                Err(UserServiceError::UserAlreadyExists)
            } else {
                debug!("Adding user {}", id);
                let user = User::new(id.clone(), name.to_string(), email.to_string());
                users.insert(id.clone(), user);
                info!("Added user {}", id);
                Ok(())
            }
        })
    }
}
