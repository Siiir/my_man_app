pub mod human {
    use actix_web::{get, HttpResponse, Responder};

    #[get("/human")]
    async fn search() -> impl Responder {
        HttpResponse::Ok().body("Human search not impl yet.")
    }
}
