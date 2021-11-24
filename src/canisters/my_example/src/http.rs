use candid::{candid_method, Principal};
use ic_cdk_macros::*;
use log::info;
use url::Url;

use ic_common::http::{HttpRequest, HttpResponse};

use crate::service::UserService;

#[query]
#[candid_method(query, rename = "http_request")]
fn http_request(req: HttpRequest) -> HttpResponse {
    info!("request: {:?}", req);
    let id = req.get_query_value("id");

    if id.is_none() {
        return HttpResponse::string(400, "id is required");
    }
    let principal = Principal::from_text(id.unwrap());
    if principal.is_err() {
        return HttpResponse::string(400, "id is invalid");
    }

    let service = UserService::new();
    let user = service.get_user_by_id(&principal.unwrap());
    if user.is_err() {
        return HttpResponse::string(400, "user not found");
    }
    HttpResponse::string(200, format!("{}", user.unwrap()).as_str())
}
