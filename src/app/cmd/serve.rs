//! Provides utilities to form & execute command that will make this app into server..

use actix_web::{web, HttpResponse, HttpServer};

/// Command that will cause app to serve its content at [`self.at_addr`].
#[derive(Clone, clap::Args)]
pub struct ServeCmd {
    #[arg(default_value = "localhost:8080")]
    at_addr: std::net::SocketAddr,
}
impl ServeCmd {
    /// Returns a socket address that should be used for serving.
    pub fn at_addr(&self) -> std::net::SocketAddr {
        self.at_addr
    }
}

impl mma::DbCommand for ServeCmd {
    type T = ();

    type E = anyhow::Error;

    fn exec_using(self, _connection: &mut diesel::MysqlConnection) -> Result<Self::T, Self::E>
    where
        Self: Sized,
    {
        // Create and run the Actix system
        let sys = actix_web::rt::System::new();
        sys.block_on(async {
            HttpServer::new(|| actix_web::App::new().route("/", web::get().to(HttpResponse::Ok)))
                .bind(self.at_addr())?
                .run()
                .await
        })?;
        Ok(())
    }
}
