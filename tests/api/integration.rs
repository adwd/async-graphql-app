use crate::helpers::get_app;
use actix_web::{http::header::ContentType, test, web};

#[actix_web::test]
async fn test_query() {
    // Arrange
    let app = get_app().await;

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
    let app = get_app().await;

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
