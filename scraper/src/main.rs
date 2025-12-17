use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;
use scraper::{Element, Html, Selector};
use serde_json;
use reqwest;
use std::collections::HashSet;

// define Story struct
#[derive(Serialize, Deserialize)]
struct Story {
    title: String,
    url: String,
    author: String,
    score: u32,
}

fn main() {
    // define the target site
    let hackernews = "https://news.ycombinator.com";

    // unwrap html content
    let content = reqwest::blocking::get(hackernews).unwrap().text().unwrap();
    fs::write("data/hackernews.html", &content).unwrap();
    let document = Html::parse_document(&content);

    // selectors for essential data
    let story_selector = Selector::parse("tr.athing").unwrap();
    let title_selector = Selector::parse("span.titleline > a").unwrap();
    let author_selector = Selector::parse(".hnuser").unwrap();
    let score_selector = Selector::parse(".score").unwrap();

    // create jsonl file or append if it already exists
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/hackernews.jsonl")
        .unwrap();
    let mut writer = std::io::BufWriter::new(file);
    let mut stories_count = 0;
    let mut url_hashes: HashSet<String> = HashSet::new();

    let jsonl_file_path = "data/hackernews.jsonl";
    if std::path::Path::new(jsonl_file_path).exists() {
        let existing_content = fs::read_to_string(jsonl_file_path).unwrap();
        for line in existing_content.lines() {
            if let Ok(story) = serde_json::from_str::<Story>(line) {
                url_hashes.insert(story.url);
            }
        }
    }


    for story_element in document.select(&story_selector) {
        let title_element = story_element.select(&title_selector).next();
        let title = title_element.map_or("N/A".to_string(), |el| el.text().collect::<String>());
        let url = title_element.and_then(|el| el.value().attr("href")).unwrap_or("").to_string();

        let mut author = "N/A".to_string();
        let mut score = 0;

        if let Some(metadata_element) = story_element.next_sibling_element() {
            if let Some(author_element) = metadata_element.select(&author_selector).next() {
                author = author_element.text().collect::<String>();
            }
            if let Some(score_element) = metadata_element.select(&score_selector).next() {
                let score_text = score_element.text().collect::<String>();
                score = score_text.split_whitespace().next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            }
        }

        if !url.is_empty() && title != "N/A" {
            // checks if story has already been saved to .jsonl file
            if url_hashes.contains(&url) {
                // if it has been saved and is a duplicate, skip this iteration and continue to the next
               continue;
            };

            // save the extracted data as a Story struct to then serialize it
            let story = Story {
                title,
                url: url.clone(),
                author,
                score,
            };

            // reads the struct to string and serializes it to json
            let json_line = serde_json::to_string(&story).unwrap();
            writeln!(writer, "{}", json_line).unwrap();
            // increments story count after each loop and add the url to the saved urls
            url_hashes.insert(url);
            stories_count += 1;
        }
    }

    println!("Successfully scraped and appended {} stories to data/hackernews.jsonl", stories_count);
 }
