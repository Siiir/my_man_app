use anyhow::Context;
use handlebars::Handlebars;

pub mod file;
pub mod human;

#[actix_web::get("/")]
pub async fn redirect_to_human() -> impl actix_web::Responder {
    actix_web::HttpResponse::PermanentRedirect()
        .insert_header(("Location", "/human"))
        .finish()
}

pub fn setup_content_templs_with_context() -> Result<Handlebars<'static>, anyhow::Error> {
    setup_content_templs().context("Failed to setup content templates.")
}
pub fn setup_content_templs() -> Result<Handlebars<'static>, handlebars::TemplateError> {
    let mut content_templs = Handlebars::new();
    content_templs.register_template_file("human", "./assets/human/index.html")?;
    Ok(content_templs)
}
