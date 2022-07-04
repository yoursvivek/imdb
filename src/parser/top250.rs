//! Module for Parsing _Top 250 Movies_.

use scraper::{ElementRef, Html, Selector};

use crate::models::{Movie, MovieBuilder, TitleID};

/// function to parse row fragment of top 250 movie html
fn parse_row_fragment(fragment: &ElementRef) -> Movie {
    let mut movie = MovieBuilder::default();

    if let Some(div) = fragment
        .select(&Selector::parse("tr > td.watchlistColumn > div.wlb_ribbon").unwrap())
        .next()
    {
        if let Some(data) = div.value().attr("data-tconst") {
            movie.id(TitleID((&data[2..]).parse::<u32>().unwrap()));
        }
    }

    if let Some(a) = fragment
        .select(&Selector::parse("tr > td.titleColumn > a").unwrap())
        .next()
    {
        if let Some(text) = a.text().next() {
            movie.title(text.trim());
        }
        /*
        if let Some(text) = a.value().attr("title") {
            movie.subtext(Some(text.trim().to_owned()));
        }
        */
    }

    for span in fragment.select(&Selector::parse("tr > td.posterColumn > span").unwrap()) {
        let element = span.value();
        match element.attr("name") {
            // ir => IMDb Rating.
            Some("ir") => {
                if let Some(value) = element.attr("data-value") {
                    movie.rating(value.parse::<f32>().unwrap());
                }
            }
            // nv => Number of Votes.
            Some("nv") => {
                if let Some(value) = element.attr("data-value") {
                    movie.votes(value.parse::<u32>().unwrap());
                }
            }
            // Two more fields `us` and `ur` are there. I've no clue what they are.
            _ => (),
        }
    }

    if let Some(span) = fragment
        .select(&Selector::parse("tr > td.titleColumn > span.secondaryInfo").unwrap())
        .next()
    {
        if let Some(text) = span.text().next() {
            movie.year((&text[1..5]).parse::<u16>().unwrap());
        }
    }

    movie.build().unwrap()
}

/// Function to Parse _Top 250 Movies_ HTML.

pub fn parse_top250_movies_html(html: &str) -> Vec<Movie> {
    let mut result = Vec::new();
    let document = Html::parse_document(html);

    let selector =
        Selector::parse("#main > div > span > div > div > div.lister > table > tbody > tr")
            .unwrap();

    for row in document.select(&selector) {
        result.push(parse_row_fragment(&row));
    }
    result
}
