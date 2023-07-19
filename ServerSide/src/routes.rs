use actix_web::web;
use actix_web::web::Data;
use crate::handlers;
use crate::models::Profile;
use crate::json_service;

pub fn configure_routes(config: &mut web::ServiceConfig) {
    let profile = json_service::get_profile_from_json().expect("failed to load profile");

    config.route("/profile",
                 web::get().to(move || handlers::return_profile(
                     profile.clone()
                 ))
    );
}