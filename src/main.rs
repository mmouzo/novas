mod theme;

use std::error::Error;

use colour::{blue, dark_green};
use dotenv::dotenv;
use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    let theme= theme::default();
    theme.print_text("# TOP TITULARES\n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("> *{}*", i.url));
        theme.print_text("-----")
      //  dark_green!("> {}\n", i.title);
      //  blue!("- {}\n\n", i.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;

    let url = "https://newsapi.org/v2/top-headlines?country=pt&apiKey=";
    let url = format!("{}{}", url, api_key);
    let articles: Articles = get_articles(&url)?;
    render_articles(&articles);
    Ok(())
}
