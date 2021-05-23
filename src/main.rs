#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    zero2prod::run()?.await
}
