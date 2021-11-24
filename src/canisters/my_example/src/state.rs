use std::cell::RefCell;
use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api, storage};
use ic_cdk_macros::*;
use log::info;

use crate::models::*;
use crate::startup::initialize;
use crate::state::models::UserStable;

mod models;

thread_local! {
    pub static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct UpgradePayloadStable {
    users: HashMap<Principal, UserStable>,
}

#[init]
fn init_function() {
    initialize();
}

#[pre_upgrade]
fn pre_upgrade() {
    match storage::stable_save((UpgradePayloadStable {
        users: USERS.with(|state| {
            let state = state.borrow();
            let mut users_stable = HashMap::new();
            for (id, user) in state.iter() {
                users_stable.insert(id.clone(), UserStable::from(user));
            }
            users_stable
        }),
    },))
    {
        Ok(_) => {
            info!("Saved state before upgrade");
            ()
        }
        Err(e) => api::trap(format!("Failed to save state before upgrade: {:?}", e).as_str()),
    }
}

#[post_upgrade]
fn post_upgrade() {
    match storage::stable_restore::<(UpgradePayloadStable,)>() {
        Ok(payload) => {
            initialize();

            info!("Start to restored state after upgrade");
            let payload = payload.0;

            let users = payload.users;
            USERS.with(|state| {
                let mut state = state.borrow_mut();
                state.clear();
                for (id, user) in users.iter() {
                    state.insert(id.clone(), User::from(user));
                }
            });
            info!("End of restored state after upgrade");
        }
        Err(e) => api::trap(format!("Failed to restored state after upgrade: {:?}", e).as_str()),
    }
}
