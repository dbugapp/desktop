use warp::{hyper::Method, Filter};


pub async fn listen() {

    let payload = warp::post()
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            println!("Received payload: {}", body); // Log the received payload
            format!("hello {}!", body) // Return the formatted string
        });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::OPTIONS])
        .allow_headers(vec!["Content-Type", "Authorization", "Accept", "Origin", "X-Requested-With"])
        .max_age(3600);

    let routes = payload.with(cors);

    println!("Server started at http://127.0.0.1:53821");

    // Use the correct run function
    warp::serve(routes).run(([127, 0, 0, 1], 53821)).await;
}
