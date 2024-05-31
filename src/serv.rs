#[actix_web::get("/")]
pub async fn redirect_to_human() -> impl actix_web::Responder {
    actix_web::HttpResponse::PermanentRedirect()
        .insert_header(("Location", "/human"))
        .finish()
}

pub mod human {
    use actix_web::{get, web, HttpResponse, Responder};

    #[get("/human")]
    async fn search(query: web::Query<crate::HumanPatternBuf>) -> impl Responder {
        let pattern = query.into_inner();
        // Debug
        let pattern = format!("{:?}", pattern);
        dbg!(&pattern);
        // crate::db::human::search(&mut crate::db::establish_connection().unwrap(), )
        // Process the pattern as needed
        HttpResponse::Ok().body(pattern)
    }
}
