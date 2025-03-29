# Build a GUI Rust app
* build a http server using the that listens on port 53821 for a POSTS
  * use the mini_http crate: https://docs.rs/mini_http/0.0.3/mini_http/
* this will accept POST to / that will be a JSON object
 . * store each JSON object using the sled crate: https://docs.rs/sled/0.34.7/sled/
* create a GUI using iced that lists the JSON objects as rows with the newest at the bottom
  * use the iced crate: https://docs.rs/iced/0.13.1/iced/
* This list will be scrollable and always default to the bottom
* Each block of JSON will have a border
* Do not modify Cargo.toml - if you need to add a dependency, ask for it
