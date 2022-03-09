use env_logger::Env;
use rss_server::configuration::get_configuration;
use rss_server::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}
