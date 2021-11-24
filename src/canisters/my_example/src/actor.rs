use candid::{candid_method, Principal};
use ic_cdk_macros::*;

use ic_common::dto::*;
use ic_common::errors::{ActorResult, ServiceError, to_actor_result};
use ic_common::ic_api::ic_caller;

use crate::models::User;
use crate::service::UserService;

#[query(name = "get_user_by_id")]
#[candid_method(query, rename = "get_user_by_id")]
fn get_user_by_id(id: Principal) -> ActorResult<User> {
    let mut service = UserService::new();
    let result = service.get_user_by_id(&id);
    to_actor_result(result)
}

// add new user with given id, name and email
#[update(name = "add_user")]
#[candid_method(update, rename = "add_user")]
fn add_user(id: Principal, name: String, email: String) -> ActorResult<()> {
    let mut service = UserService::new();
    let result = service.add_user(&id, &name, &email);
    to_actor_result(result)
}


candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
