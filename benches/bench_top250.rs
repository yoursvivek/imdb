#![feature(test)]
extern crate test;

use glob::glob;
use imdb::unstable::parser;
use std::fs::File;
use std::io::prelude::*;

#[bench]
fn bench_parse_top250_movies_html(b: &mut test::Bencher) {
    for entry in glob("fixtures/top250movie*.html").expect(
        "No file matching `fixtures/top250movies*.html` found.\
                Download IMDb top 250 page.",
    ) {
        match entry {
            Ok(path) => {
                let mut html = String::with_capacity(800 * 1024);
                let mut file = File::open(path).unwrap();
                file.read_to_string(&mut html).unwrap();

                b.iter(|| parser::top250::parse_top250_movies_html(&html));
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
