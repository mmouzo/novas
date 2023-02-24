use std::error::Error;

use colour::{blue, dark_green};
use serde::Deserialize;

#[derive(Deserialize)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        blue!("- {}\n\n", i.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=API_KEY";
    let articles: Articles = get_articles(url)?;
    render_articles(&articles);
    Ok(())
}
