use url::Url;
use reqwest;
use std::io::Read;
use hyper::header::{Headers, UserAgent, AcceptLanguage};

use consts;
use parser::top250;
use error::Error;
use models::Movie;
use language::Language;

/// Client to retrieve information from IMDb.

pub struct IMDb {
    /// Reqwest client
    client: reqwest::Client,
    /// Language
    lang: Option<AcceptLanguage>,
    /// User-Agent
    ua: Option<UserAgent>,
}

impl Default for IMDb {
    fn default() -> Self {
        IMDb {
            client: reqwest::Client::new(),
            lang: None,
            ua: None,
        }
    }
}

impl IMDb {
    /// Creates a new IMDb Client instance.
    pub fn new() -> Self {
        IMDb::default()
    }

    /// Sets language  for HTTP requests.
    pub fn accept_language(&mut self, lang: Language) -> &mut Self {
        self.lang = Some(lang.accept_language_header());
        self
    }

    /// Sets _User-Agent_ for HTTP requests.
    pub fn user_agent(&mut self, ua: UserAgent) -> &mut Self {
        self.ua = Some(ua);
        self
    }

    /// Makes HTTP requests.
    fn get(&self, path: &str) -> Result<reqwest::Response, Error> {
        let base = Url::parse(consts::BASE_URL).unwrap();
        let url = base.join(path).unwrap();

        let mut headers = Headers::new();

        if let Some(ref lang) = self.lang {
            headers.set(lang.clone());
        }

        if let Some(ref ua) = self.ua {
            headers.set(ua.clone());
        }

        info!("Sending HTTP request for `{}`.", url);
        self.client.get(url).headers(headers).send().map_err(
            Error::ReqwestError,
        )
    }

    /// Get _Top 250 Movies_.
    pub fn top250_movies(&self) -> Result<Vec<Movie>, Error> {

        let mut response = self.get("chart/top")?;

        let mut html = String::with_capacity(800 * 1024);
        response.read_to_string(&mut html).map_err(Error::IOError)?;

        let movies = top250::parse_top250_movies_html(&html);

        Ok(movies)
    }
}
