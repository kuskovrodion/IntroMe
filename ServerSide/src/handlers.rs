use actix_web::{web, HttpResponse, Responder};
use crate::models::Profile;

pub async fn return_profile(profile: Profile) -> impl Responder {
    HttpResponse::Ok().json(profile)
}