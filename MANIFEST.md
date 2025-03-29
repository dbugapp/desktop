# Build a GUI Rust app
* build an http server using the that listens on port 53821 for a POSTS
  * use the mini_http crate: https://crates.io/crates/mini_http
* this will accept POST to / that will be a JSON object
 . * store each JSON object using the sled crate: https://crates.io/crates/sled
* create a GUI using iced that lists the JSON objects as rows with the newest at the bottom
  * use the iced crate: https://crates.io/crates/iced
* This list will be scrollable and always default to the bottom
* Each block of JSON will have a border and be syntax highlighted