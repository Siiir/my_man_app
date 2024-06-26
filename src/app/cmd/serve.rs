//! Provides utilities to form & execute command that will make this app into server..

use actix_web::{web, HttpServer};
use anyhow::Context;

use std::{
    future::Future,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

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
        self.running_server_using_actix()?
            .await
            .context("The server has encountered I/O error, while running.")
    }
    fn running_server_using_actix(
        self,
    ) -> anyhow::Result<impl Future<Output = std::io::Result<()>>> {
        let server = (|| -> anyhow::Result<_> {
            let db_pool = web::Data::new(mma::db::establish_connection_pool_with_context()?);
            let content_templs = web::Data::new(mma::serv::setup_content_templs_with_context()?);
            let server = HttpServer::new(move || {
                let app = actix_web::App::new();
                use mma::serv::*;
                app.app_data(db_pool.clone())
                    .app_data(content_templs.clone())
                    .service(redirect_to_human)
                    .service(mma::static_file_service::<&str>(
                        "text/css; charset=UTF-8",
                        "assets/human/style.css",
                    ))
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
        .context("Failed to setup a server.")?;
        Ok(server.run())
    }
}
