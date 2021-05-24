use std::net::TcpListener;

#[actix_web::main]
pub async fn main() -> Result<(), std::io::Error> {
    zero2prod::run(TcpListener::bind("127.0.0.1:8000")?)?.await
}
