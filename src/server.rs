use iced::futures::Stream;
use iced::stream;
use iced::futures::channel::mpsc;
use serde_json::Value;
use warp::{hyper::Method, Filter};
use crate::storage::Storage;
use iced::futures::SinkExt; // Import SinkExt
use crate::gui::{Message}; // Import from gui.rs

#[derive(Debug, Clone)]
pub enum ServerMessage { // The server event types
    PayloadReceived(Value),
}


pub(crate) enum ServerInput { // Make non-public if needed only for the server
    DoSomeWork,
}


 pub fn listen() -> impl Stream<Item = ServerMessage> {

     stream::channel(100, |mut output| async move {
         let payload = warp::post()
             .and(warp::body::json())
             .map({
                 let output = output.clone();
                 move |body: Value| {
                     let mut output_clone = output.clone();
                     tokio::task::spawn(async move {
                         let _ = output_clone.send(ServerMessage::PayloadReceived(body)).await;
                     });
                     "Hello!".to_string()
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
