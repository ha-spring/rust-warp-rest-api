use warp::Filter;

#[tokio::main]
async fn main() {
  let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!\n", name));

  warp::serve(hello)
    .run(([127, 0, 0, 1], 8081))
    .await;
}
