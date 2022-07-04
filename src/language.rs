use std::fmt;

/// Options for IMDb language support via _Accept-Language_
///
/// IMDb shows content in various languages based on `Accept-Language`
/// header present in HTTP request. This library supports fetching
/// content in different languages.
///
/// This list is just a start. If there are other languages in which
/// IMDb shows content but no options are provided here, open an issue.
///
/// # Example
///
/// ```ignore,rust
/// use imdb::{IMDb, Language};
///
/// let imdb = IMDb::new();
/// imdb.accept_language(Languages::fr_FR)
///
/// top_250 = imdb.top250_movies().unwrap();
///
/// assert_eq!("Les évadés", top_250[0].title);
/// assert_eq!("Le parrain", top_250[1].title);
/// ```
///
/// # Why Typed?
///
/// Or why not stringly-typed?
///
/// Under the hood, this library uses [hyper][hyper] which is highly opiniated
/// about how `headers` should be presented.
///
/// > Hyper has the opinion that Headers should be strongly-typed, because
/// > that's why we're using Rust in the first place. To set or get any
/// > header, an object must implement the Header trait from this module.
///
/// In the same spirit, this library restricts string based options.
///
/// # References
///
/// * Tags for Identifying Languages [BCP47]
/// * HTTP/1.1 Semantics and Content # Accept-Language [RFC7231]
///
/// [hyper]: https://hyper.rs/hyper/master/hyper/index.html
/// [BCP47]: https://tools.ietf.org/html/bcp47
/// [RFC7231]: http://tools.ietf.org/html/rfc7231#section-5.3.5
#[allow(non_camel_case_types)]
pub enum Language {
    /// American English
    en_US,
    /// British English
    en_GB,
    /// Italian (Italy)
    it_IT,
    /// French (France)
    fr_FR,
    /// German (German)
    de_DE,
    /// Danish (Denmark)
    da_DK,
}

impl Language {
    /// Returns Accept-Language Header of Language Option
    pub fn accept_language_header(&self) -> String {
        let language_tag = match *self {
            Language::en_US => "en-US",
            Language::en_GB => "en-GB",
            Language::fr_FR => "fr-FR",
            Language::de_DE => "de-DE",
            Language::da_DK => "da-DK",
            Language::it_IT => "it-IT",
        };
        language_tag.to_string()
    }

    /// Returns Description of Language Option
    pub fn description(&self) -> &'static str {
        match *self {
            Language::en_US => "American English",
            Language::en_GB => "British English",
            Language::fr_FR => "French (France)",
            Language::de_DE => "German (Germany)",
            Language::da_DK => "Danish (Denmark)",
            Language::it_IT => "Italian (Italy)",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            Language::en_US => "en-US",
            Language::en_GB => "en-GB",
            Language::fr_FR => "fr-FR",
            Language::de_DE => "de-DE",
            Language::da_DK => "da-DK",
            Language::it_IT => "it-IT",
        };
        write!(f, "{}", display.to_string())
    }
}
