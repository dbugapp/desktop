use mini_server::{Server, Request, Response};
use std::sync::Arc;
use std::sync::Mutex;
use crate::storage::Storage;

pub fn start() {
    let storage = Arc::new(Mutex::new(Storage::new()));

    let server = Server::new("127.0.0.1:54321", move |req: Request| {
        if req.method() == "POST" && req.path() == "/" {
            let body = req.body();
            let json_object: serde_json::Value = serde_json::from_slice(&body).unwrap();
            storage.lock().unwrap().add(json_object);
            Response::new(200, "OK")
        } else {
            Response::new(404, "Not Found")
        }
    });

    server.run().unwrap();
}
