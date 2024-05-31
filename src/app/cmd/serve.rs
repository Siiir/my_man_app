//! Provides utilities to form & execute command that will make this app into server..

use actix_web::{web, HttpServer};
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
        sys.block_on(self.serve_using_actix_with_context())
    }

    async fn serve_using_actix_with_context(self) -> anyhow::Result<()> {
        self.serve_using_actix()
            .await
            .context("The server has encountered I/O error, while running.")
    }
    async fn serve_using_actix(self) -> anyhow::Result<()> {
        let res = (|| -> anyhow::Result<_> {
            let db_pool = web::Data::new(mma::db::establish_connection_pool()?);
            let server = HttpServer::new(move || {
                let app = actix_web::App::new();
                use mma::serv::*;
                app.app_data(db_pool.clone())
                    .service(redirect_to_human)
                    .service(human::search)
            })
            .bind(self.host_addr())
            .with_context(|| {
                format!(
                    "The server has encountered I/O errror, while trying to bind to {:}.",
                    self.host_addr()
                )
            })?;
            Ok(server)
        })()
        .context("Failed to setup a server.")?
        .run()
        .await;
        Ok(res?)
    }
}
