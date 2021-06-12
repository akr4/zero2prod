use sqlx::{Connection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
pub async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to Postgres.");

    run(
        TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?,
        connection_pool,
    )?
    .await
}
