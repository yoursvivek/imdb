use log::{info, warn};

use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

use colored::*;
use terminal_size::{terminal_size, Height, Width};

use imdb::unstable::parser;
use imdb::{Language, Movie};

fn format_fields(
    movie: &Movie,
    max_title_length: usize,
) -> (
    ColoredString,
    String,
    ColoredString,
    ColoredString,
    ColoredString,
) {
    let id = format!("{}", movie.id).white();
    let title = format!("{:width$}", movie.title.blue(), width = max_title_length);
    let year = format!("{}", movie.year).yellow();
    let rating = format!("{:.1}", movie.rating).green();
    let votes = format!("{:7}", movie.votes).cyan();
    (id, title, year, rating, votes)
}

fn print_one_line(movies: &[Movie], max_title_length: usize) {
    for (i, movie) in movies.iter().enumerate() {
        let (id, title, year, rating, votes) = format_fields(&movie, max_title_length);
        println!(
            "[{}] {:3} {} ({}) {} {} votes",
            id,
            i + 1,
            title,
            year,
            rating,
            votes,
        );
    }
}

fn print_two_lines(movies: &[Movie], max_title_length: usize) {
    for (i, movie) in movies.iter().enumerate() {
        let (id, title, year, rating, votes) = format_fields(&movie, max_title_length);
        println!("{:3} {} ({})", i + 1, title, year,);
        println!(
            "{}",
            format!(
                "[{}] {:>width$} {} votes",
                id,
                rating,
                votes,
                width = max_title_length - 16,
            )
            .on_black(),
        );
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = ::std::env::args().collect();

    let movies = match args.len() {
        2 => {
            let mut html = String::with_capacity(800 * 1024);

            let mut file = File::open(&args[1]).unwrap();
            file.read_to_string(&mut html).unwrap();

            parser::top250::parse_top250_movies_html(&html)
        }
        _ => {
            let mut imdb = imdb::IMDb::new();
            imdb.accept_language(Language::en_GB);
            imdb.top250_movies().await.unwrap()
        }
    };

    info!("{} movies found.", movies.len());

    if !movies.is_empty() {
        let max_title_length: usize = movies
            .iter()
            .map(|x| x.title.chars().count())
            .max()
            .unwrap();
        info!("Maximum length of title is {}.", max_title_length);

        let size = terminal_size();

        if let Some((Width(w), Height(_))) = size {
            info!("Terminal width detected as {} columns.", w);
            if max_title_length + 41 > w as usize {
                info!("Selecting two line layout.");
                print_two_lines(&movies, max_title_length);
            } else {
                info!("Selecting one line layout.");
                print_one_line(&movies, max_title_length);
            }
        } else {
            info!("Terminal width not known. Selecting one line layout as fallback.");
            print_one_line(&movies, max_title_length);
        }
    } else {
        warn!("No movies found. Something horribly wrong is going on!");
    }
    Ok(())
}
