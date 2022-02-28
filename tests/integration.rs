use actix_web::{guard, http::header::ContentType, test, web, App};
use async_graphql_app::{get_schema, index, index_playground};

#[actix_web::test]
async fn test_query() {
    // Arrange
    let schema = get_schema();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(schema))
            .service(web::resource("/").guard(guard::Post()).to(index)),
    )
    .await;

    // Act
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_payload(r#"{"query":"{__typename}"}"#)
        .to_request();
    let resp = test::call_and_read_body(&app, req).await;

    // Assert
    assert_eq!(resp, web::Bytes::from(r#"{"data":{"__typename":"Query"}}"#));
}

#[actix_web::test]
async fn test_playground() {
    // Arrange
    let schema = get_schema();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(schema))
            .route("/", web::get().to(index_playground)),
    )
    .await;

    // Act
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    assert!(resp
        .response()
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
        .contains("text/html"));
}
