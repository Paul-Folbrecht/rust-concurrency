use reqwest::{self};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let url = "http://api.apis.guru/v2/list.json";
    let mut set = JoinSet::new();

    for _ in 0..10 {
        set.spawn(async move { request(url) });
    }

    while let Some(res) = set.join_next().await {
        println!("Result: {}", res.unwrap().await);
    }
}

async fn request(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(response) => response.status().to_string(),
        Err(error) => error.to_string(),
    }
}
