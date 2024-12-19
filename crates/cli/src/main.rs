use std::io;

use rss_parser::Channel;

#[tokio::main]
async fn main() {
    let mut feed_url = String::new();

    println!("Please input the feed URL:");

    io::stdin()
        .read_line(&mut feed_url)
        .expect("Failed to get feed URL");

    let content = reqwest::get(feed_url).await.unwrap().text().await.unwrap();
    let channel = Channel::from(content.as_str());

    dbg!(channel);
}
