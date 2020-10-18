#[macro_use]
extern crate log;
extern crate env_logger;

extern crate imdb;
extern crate colored;
extern crate terminal_size;

use std::io::Read;
use std::fs::File;
use std::iter::Iterator;

use futures::executor::block_on;

use terminal_size::{Width, Height, terminal_size};
use colored::*;

use crate::imdb::unstable::parser;
use crate::imdb::{Movie, Language};

fn print_one_line(movies: &[Movie], max_title_length: usize) {
    for (i, movie) in movies.iter().enumerate() {
        println!("[{}] {} {} ({}) {} {} votes",
            format!("{}", movie.id),
            format!("{:3}", i+1).white(),
            format!("{:width$}", movie.title.blue(), width=max_title_length),
            format!("{}", movie.year).yellow(),
            format!("{:.1}", movie.rating).green(),
            format!("{:7}", movie.votes).cyan(),
        );
    }
}

fn print_two_lines(movies: &[Movie], max_title_length: usize) {
    for (i, movie) in movies.iter().enumerate() {
        println!("{} {} ({})",
                 format!("{:3}", i+1).white(),
                 format!("{:width$}", movie.title.blue(), width=max_title_length),
                 format!("{}", movie.year).yellow(),
        );
        println!("{}",
            format!("[{}] {:>width$} {} votes",
                    format!("{}", movie.id),
                    format!("{:.1}", movie.rating).green(),
                    format!("{:7}", movie.votes).cyan(),
                    width = max_title_length - 15,
            ).on_black(),
        );
    }
}

fn main() {
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
            imdb.accept_language(Language::da_DK);
            block_on(imdb.top250_movies()).unwrap()
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
}
