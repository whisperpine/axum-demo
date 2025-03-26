use super::*;

#[tokio::test]
async fn test_log_path() {
    use axum::body::{to_bytes, Body};
    use axum::http::Request;
    use axum::routing::get;
    use tower::ServiceExt; // For .oneshot()

    let app = axum::Router::new().route("/path/{path_id}", get(log_path));

    let request = Request::builder()
        .uri("/path/42")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_str, "42");
}
