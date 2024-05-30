//! Provides utilities to form & execute command that will make this app into server..

use actix_web::{web, HttpResponse, HttpServer};
use anyhow::Context;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Command that will cause app to serve its content at the specified address and port.
#[derive(Clone, clap::Args)]
pub struct ServeCmd {
    /// Host address to bind the server to.
    #[arg(short, long, default_value_t = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)))]
    ip_addr: IpAddr,

    /// Port to bind the server to.
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

impl ServeCmd {
    // CRUD-R: Properties

    /// Returns a socket address that should be used for serving.
    pub fn host_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip_addr, self.port)
    }

    // CRUD-D: Consuming usage.

    /// Executes command [`self`] by running a server.
    pub fn exec(self) -> anyhow::Result<()> {
        // Creating actix runtime.
        let sys = actix_web::rt::System::new();
        // Running actix server.
        sys.block_on(self.serve_using_actix())
    }

    async fn serve_using_actix(self) -> anyhow::Result<()> {
        HttpServer::new(|| actix_web::App::new().route("/", web::get().to(HttpResponse::Ok)))
            .bind(self.host_addr())
            .with_context(|| {
                format!(
                    "The server has encountered I/O errror, while trying to bind to {:}.",
                    self.host_addr()
                )
            })?
            .run()
            .await
            .context("The server has encountered I/O error, while running.")
    }
}
