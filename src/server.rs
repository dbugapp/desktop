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
    Ready(mpsc::Sender<ServerInput>),
    PayloadReceived(Value),
    WorkFinished,
}


pub(crate) enum ServerInput { // Make non-public if needed only for the server
    DoSomeWork,
}


 pub fn listen() -> impl Stream<Item = ServerMessage> {

     stream::channel(100, |mut output| async move {
         let storage = Storage::new().expect("Failed to initialize storage");

         let payload = warp::post()
             .and(warp::body::json())
             .map({
                 let storage = storage.clone();
                 let mut output = output.clone();
                 move |body: Value| {
                     println!("Received payload: {}", body);

                     let storage = storage.clone();
                     let mut output_clone = output.clone();

                     tokio::task::spawn(async move {
                         if let Err(e) = storage.add_json(&body) {
                             eprintln!("Failed to store payload: {}", e);
                         }
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
