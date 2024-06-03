#[actix_web::get("/")]
pub async fn redirect_to_human() -> impl actix_web::Responder {
    actix_web::HttpResponse::PermanentRedirect()
        .insert_header(("Location", "/human"))
        .finish()
}

pub mod human;
