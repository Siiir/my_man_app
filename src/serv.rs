#[actix_web::get("/")]
pub async fn redirect_to_human() -> impl actix_web::Responder {
    actix_web::HttpResponse::PermanentRedirect()
        .insert_header(("Location", "/human"))
        .finish()
}

pub mod human {
    use actix_web::{get, web, HttpResponse, Responder};
    use anyhow::Context;

    #[get("/human")]
    async fn search(
        query: web::Query<crate::HumanPatternBuf>,
        db_pool: web::Data<crate::DbPool>,
    ) -> impl Responder {
        let pattern = query.into_inner();

        // Log
        tracing::info!("Received human query with filter: {pattern}");

        // The search
        let mut connection = db_pool
            .get()
            .context("No free database connection in the pool.")
            .unwrap();
        let ret = crate::db::human::search(&mut connection, pattern).unwrap();

        // Req. result
        HttpResponse::Ok().body(tabled::Table::new(ret).to_string())
    }
}
