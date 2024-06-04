pub mod query;
pub use search_fn::search;
pub mod search_fn {
    use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
    use anyhow::Context;
    use handlebars::Handlebars;

    #[get("/human")]
    pub async fn search(
        // Client query
        http_req: HttpRequest,
        query: web::Query<crate::HumanHttpQuery>,
        // Data
        db_pool: web::Data<crate::DbPool>,
        content_templs: web::Data<Handlebars<'static>>,
    ) -> impl Responder {
        let pattern: crate::HumanPatternBuf = query.into_inner().into();
        let user_ip = http_req
            .peer_addr()
            .map(|addr| (addr.to_string().into()))
            .unwrap_or(std::borrow::Cow::Borrowed("unknown"));
        // Log
        tracing::info!("Received human query with filter \"{pattern}\" from ip {user_ip}.");
        // Perform
        match raw_search(pattern, &db_pool, &content_templs)
            .await
            .with_context(|| format!("Failed to perform request."))
        {
            Ok(ret) => ret,
            Err(err) => HttpResponse::InternalServerError().body(format!("<pre>{err}</pre>")),
        }
    }

    async fn raw_search(
        pattern: crate::HumanPatternBuf,
        // Data
        db_pool: &crate::DbPool,
        content_templs: &Handlebars<'static>,
    ) -> anyhow::Result<HttpResponse> {
        // The search
        let mut connection = crate::db::connect_using_pool_with_context(db_pool)?;
        let humans_found = crate::db::human::search_with_context(&mut connection, pattern.clone())?;

        // Req. result
        Ok(page_showing(pattern.into(), content_templs, humans_found))
    }

    fn page_showing(
        human_query: crate::HumanHttpQuery,
        content_templs: &Handlebars<'static>,
        humans: Vec<crate::models::Human>,
    ) -> HttpResponse {
        let mut tabled_humans = tabled::Table::new(humans);
        let tabled_humans = tabled_humans.with(tabled::settings::Style::dots());
        let html_table = format!("<pre>{tabled_humans}</pre>");

        let mut data = serde_json::json!({
            "humans_found": html_table,
        });

        let human_query_json = serde_json::to_value(human_query).unwrap();
        if let serde_json::Value::Object(ref mut map) = data {
            map.extend(human_query_json.as_object().unwrap().clone());
        }

        let page_content = content_templs.render("human", &data).unwrap();
        HttpResponse::Ok()
            .content_type("text/html; charset=UTF-8")
            .body(page_content)
    }
}
