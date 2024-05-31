use anyhow::Context;
use tracing_subscriber::FmtSubscriber;

/// Initializes this app.
///
/// Makes sure that all environment stuff is accessible in standard way.
pub fn all() {
    // Use of .env file.
    _ = dotenvy::dotenv();

    // Create a filter that sets the log level for actix to info and the rest to trace
    let env_filter = tracing_subscriber::EnvFilter::new(
        "actix_web=info,actix_server=info,actix_http=info,trace",
    );

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .finish();

    match tracing::subscriber::set_global_default(subscriber).context(
        "Correct logs may not be produced, because app could not set global log subscriber.",
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("WARNING: {e}");
        }
    }
}
