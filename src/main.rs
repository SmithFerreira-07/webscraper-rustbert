use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use rust_bert::pipelines::sentiment::{SentimentModel, Sentiment};
use std::error::Error;
use std::io::{self, Write};

fn scrape_links(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send()?.text()?;

    
    let document = Document::from(res.as_str());

    
    println!("Links found on the page:");
    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            println!("Link: {}", href);
        }
    }

    Ok(())
}

fn analyze_sentiment(text: &str) -> Result<Vec<Sentiment>, Box<dyn Error>> {
    let model = SentimentModel::new(Default::default())?;

    let sentiment_results = model.predict(&[text]);
    Ok(sentiment_results)
}

fn main() -> Result<(), Box<dyn Error>> {

    print!("Enter the URL to scrape: ");
    io::stdout().flush()?;

    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim();

    
    match scrape_links(url) {
        Ok(()) => println!("Scraping completed successfully."),
        Err(e) => eprintln!("Error during scraping: {}", e),
    }

    
    let example_text = "This is a great example!";
    match analyze_sentiment(example_text) {
        Ok(sentiments) => {
            for sentiment in sentiments {
                println!("{:?}", sentiment);
            }
        }
        Err(e) => eprintln!("Error during sentiment analysis: {}", e),
    }

    Ok(())
}

