//! *imdb* is a Rust library to retrieve information from [IMDb].
//!
//! # Example
//!
//! ```ignore,rust
//! # extern crate imdb;
//! use imdb::IMDb;
//!
//! imdb = IMDb::new();
//! top250movies = imdb.top250_movies().unwrap()
//! ```
//!
//! [IMDb]: http://www.imdb.com/

#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_builder;
extern crate hyper;
#[macro_use]
extern crate language_tags;
extern crate reqwest;
extern crate scraper;
extern crate url;

#[cfg(feature = "serde-impls")]
#[macro_use]
extern crate serde_derive;
//#[cfg(feature = "serde-impls")]
//extern crate serde;

mod consts;
mod parser;

mod error;
mod imdb;
mod language;

pub mod models;

pub use error::Error;
pub use language::Language;
pub use imdb::IMDb;
pub use models::Movie;

pub mod unstable {
    //! Unstable Internal APIs.
    //!
    //! # Avalability
    //! These modules features unstable internal parser not available as part of general public api.
    //! There are no guarantees on backward compatibility on this module.
    //!
    //! You have been warned!
    //!
    //! # What sort of changes?
    //!
    //! Various assumptions have been made on HTML DOM of respective pages. Changes will be done to
    //! remove those assumption if they don't hold good. Changes will also be done to add more
    //! parsers for additional information, or at times possibly using different page for same
    //! information.

    pub mod parser {
        //! Parsers for extracting information from IMDb html pages.

        pub use parser::top250;
    }
}
