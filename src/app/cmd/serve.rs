//! Provides utilities to form & execute command that will make this app into server..

use actix_web::{web, HttpResponse, HttpServer};

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
    /// Returns a socket address that should be used for serving.
    pub fn host_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip_addr, self.port)
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
                .bind(self.host_addr())?
                .run()
                .await
        })?;
        Ok(())
    }
}
