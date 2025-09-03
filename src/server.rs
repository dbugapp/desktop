use iced::futures::Stream;
use iced::stream;
use serde_json::Value;
use warp::{hyper::Method, Filter};
use iced::futures::SinkExt;
use crate::settings::Settings;

#[derive(Debug, Clone)]
pub enum ServerMessage {
    PayloadReceived(Value),
}


pub(crate) enum _ServerInput {
    DoSomeWork,
}


 pub fn listen() -> impl Stream<Item = ServerMessage> {

     stream::channel(100, |output: futures::channel::mpsc::Sender<ServerMessage>| async move {
         let settings = Settings::load();
         let host = settings.get_server_host();
         let port = settings.get_server_port();
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

         println!("Server started at http://{host}:{port}");

         let addr: std::net::SocketAddr = format!("{host}:{port}")
             .parse()
             .unwrap_or_else(|_| ([127, 0, 0, 1], 53821).into());

         warp::serve(routes).run(addr).await;

     })
}
