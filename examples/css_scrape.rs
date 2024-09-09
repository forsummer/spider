//! `cargo run --example css_scrape`
extern crate spider;

use spider::tokio;
use spider::website::Website;
use spider_utils::{
    build_selectors, css_query_select_map_streamed, QueryCSSMap, QueryCSSSelectSet,
};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut website: Website = Website::new("https://spider.cloud")
        .with_limit(5)
        .build()
        .unwrap();
    let mut rx2: tokio::sync::broadcast::Receiver<spider::page::Page> =
        website.subscribe(0).unwrap();
    let mut stdout = tokio::io::stdout();

    let join_handle = tokio::spawn(async move {
        while let Ok(res) = rx2.recv().await {
            let map = QueryCSSMap::from([("list", QueryCSSSelectSet::from(["ul", "ol"]))]);

            let data = css_query_select_map_streamed(&res.get_html(), &build_selectors(map)).await;

            let _ = stdout.write_all(format!("- {:?}\n", data).as_bytes()).await;
        }
        stdout
    });

    let start = std::time::Instant::now();
    website.crawl().await;
    website.unsubscribe();
    let duration = start.elapsed();
    let mut stdout = join_handle.await.unwrap();

    let _ = stdout
        .write_all(
            format!(
                "Time elapsed in website.crawl() is: {:?} for total pages: {:?}",
                duration,
                website.get_links().len()
            )
            .as_bytes(),
        )
        .await;
}
