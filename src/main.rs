extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() {
    print!("Data: ");
    get_data("https://doc.rust-lang.org/rust-by-example/hello/print.html").await;
}

async fn get_data(url: &str) {
    let request = reqwest::get(url).await.unwrap();
    assert!(request.status().is_success());
    let body = Html::parse_document(&request.text().await.unwrap());

    let data = Selector::parse(".menu-title").unwrap();

    for element in body.select(&data) {
        let data = element.text().collect::<Vec<_>>();
        println!("{:}", data[0]);
    }
}
