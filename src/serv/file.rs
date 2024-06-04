use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use actix_web::{dev::HttpServiceFactory, web, HttpResponse};

pub fn static_file_service<P>(
    content_type: &'static str,
    file_path: &'static str,
) -> impl HttpServiceFactory + 'static
where
    P: AsRef<Path>,
{
    file_service::<&str, _>(content_type, file_path, core::convert::identity)
}

pub fn file_service<P, F>(
    content_type: &'static str,
    file_path: &'static str,
    adjust_resp: F,
) -> impl HttpServiceFactory + 'static
where
    P: AsRef<Path>,
    F: (Fn(String) -> String) + Copy + 'static,
{
    web::resource(file_path).get(move || serve_file(content_type, file_path, adjust_resp))
}

pub async fn serve_file<P, F>(content_type: &str, file_path: P, adjust_resp: F) -> HttpResponse
where
    P: AsRef<Path>,
    F: FnOnce(String) -> String,
{
    #[cached::proc_macro::cached(
        time = 60,
        size = 4092,
        convert = "{file_path.to_owned()}",
        key = "PathBuf"
    )]
    async fn get_content(file_path: &Path) -> Result<String, Arc<std::io::Error>> {
        match fs_err::tokio::read_to_string(file_path).await {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(e.into()),
        }
    }
    let file_path = file_path.as_ref();
    match get_content(&file_path).await.map(adjust_resp) {
        Ok(content) => HttpResponse::Ok().content_type(content_type).body(content),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
