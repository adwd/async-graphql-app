use actix_http::{body::BoxBody, Request};
use actix_service::Service;
use actix_web::{dev::ServiceResponse, test, web, App, Error};
use async_graphql_app::{get_schema, index, index_playground};

pub async fn get_app() -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = Error>
{
    let schema = get_schema();

    test::init_service(
        App::new()
            .app_data(web::Data::new(schema))
            .route("/", web::post().to(index))
            .route("/", web::get().to(index_playground)),
    )
    .await
}
