use warp::{http::StatusCode, Filter};

pub async fn listen() {
    // Extract the body first
    let payload = warp::post()
        .and(warp::path("hello"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|body: serde_json::Value| format!("hello {}!", body));

    println!("Server started at http://127.0.0.1:53821");

    // Use the correct run function
    warp::serve(payload).run(([127, 0, 0, 1], 53821)).await;
}
