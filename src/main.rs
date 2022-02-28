use async_graphql_app::get_server;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let server = get_server()?;

    server.await
}
