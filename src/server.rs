use iced::futures::Stream;
use iced::stream;
use iced::futures::channel::mpsc;

use warp::{hyper::Method, Filter};
use crate::storage::Storage;
use crate::gui::{Message}; // Import from gui.rs



pub(crate) enum ServerInput { // Make non-public if needed only for the server
    DoSomeWork,
}


 pub fn listen() -> impl Stream<Item = Message> {

     stream::channel(100, |mut output| async move {
         let storage = Storage::new().expect("Failed to initialize storage");

         let payload = warp::post()
             .and(warp::body::json())
             .map({
                 let storage = storage.clone();
                 move |body: serde_json::Value| {
                     println!("Received payload: {}", body); // Log the received payload
                     // Store the payload
                     if let Err(e) = storage.add_json(body.clone()) {
                         eprintln!("Failed to store payload: {}", e);
                     }
                     format!("hello {}!", body) // Return the formatted string
                 }
             });

         let cors = warp::cors()
             .allow_any_origin()
             .allow_methods(&[Method::POST, Method::OPTIONS])
             .allow_headers(vec!["Content-Type", "Authorization", "Accept", "Origin", "X-Requested-With"])
             .max_age(3600);

         let routes = payload.with(cors);

         println!("Server started at http://127.0.0.1:53821");

         warp::serve(routes).run(([127, 0, 0, 1], 53821)).await;

     })
}
