pub use search_fn::search;
pub mod search_fn {
    use std::future::Future;

    use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
    use anyhow::Context;

    #[get("/human")]
    pub async fn search(
        http_req: HttpRequest,
        query: web::Query<crate::HumanPatternBuf>,
        db_pool: web::Data<crate::DbPool>,
    ) -> impl Responder {
        let pattern = query.into_inner();
        let user_ip = http_req
            .peer_addr()
            .map(|addr| (addr.to_string().into()))
            .unwrap_or(std::borrow::Cow::Borrowed("unknown"));
        // Log
        tracing::info!("Received human query with filter \"{pattern}\" from ip {user_ip}.");
        // Perform
        match raw_search(pattern, &db_pool)
            .await
            .with_context(|| format!("Failed to perform request."))
        {
            Ok(ret) => ret,
            Err(err) => HttpResponse::InternalServerError().body(format!("<pre>{err}</pre>")),
        }
    }

    async fn raw_search(
        pattern: crate::HumanPatternBuf,
        db_pool: &crate::DbPool,
    ) -> anyhow::Result<HttpResponse> {
        // The search
        let mut connection = crate::db::connect_using_pool_with_context(db_pool)?;
        let humans_found = crate::db::human::search_with_context(&mut connection, pattern)?;

        // Req. result
        Ok(page_showing(humans_found).await)
    }

    fn page_showing(humans: Vec<crate::models::Human>) -> impl Future<Output = HttpResponse> {
        let mut tabled_humans = tabled::Table::new(humans);
        let tabled_humans = tabled_humans.with(tabled::settings::Style::dots());
        let html_table = format!("<pre>{tabled_humans}</pre>");

        crate::serve_file(
            "text/html; charset=UTF-8",
            "assets/human/index.html",
            move |content| content.replace("{html_table}", &html_table),
        )
    }
}
