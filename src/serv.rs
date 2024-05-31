pub mod human {
    use actix_web::{get, web, HttpResponse, Responder};

    #[get("/human")]
    async fn search(query: web::Query<crate::HumanPatternBuf>) -> impl Responder {
        let pattern = query.into_inner();
        // crate::db::human::search(&mut crate::db::establish_connection().unwrap(), )
        // Process the pattern as needed
        HttpResponse::Ok()
    }
}
