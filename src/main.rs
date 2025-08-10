use crate::app::http;
use crate::app::params::HttpParams;

mod app;

#[tokio::main]
async fn main() {
    http::run(HttpParams::default()).await;
}
