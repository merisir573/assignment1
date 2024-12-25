use actix_web::{web, App, HttpResponse, HttpServer};
use reqwest::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Instant;

async fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url).await?.text().await?;
    Ok(response)
}

fn parse_page(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("h3 a").unwrap();
    let titles = document
        .select(&selector)
        .map(|x| x.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<String>>();
    titles
}

async fn scrape() -> HttpResponse {
    let start = Instant::now();
    let base_url = "http://books.toscrape.com/catalogue/page-";
    let mut all_titles = Vec::new();
    let mut page = 1;
    
    loop {
        let url = &format!("{}{}.html", base_url, page);
        match fetch_page(url).await {
            Ok(html) => {
                let titles = parse_page(&html);
                if titles.is_empty() {
                    break; 
                }
                all_titles.extend(titles);
            }
            Err(_) => break,
        }
        page += 1;
    }
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    
    HttpResponse::Ok().json(all_titles)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/scrape", web::get().to(scrape))
    })
    .bind("0.0.0.0:6000")?
    .run()
    .await
}
