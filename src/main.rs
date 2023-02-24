use std::error::Error;

use colour::{blue, dark_green};
use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        blue!("- {}\n\n", i.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let url =
        "https://newsapi.org/v2/top-headlines?country=pt&apiKey=9fee85a9887843f192e3323606119a23";
    let articles: Articles = get_articles(url)?;
    render_articles(&articles);
    Ok(())
}
