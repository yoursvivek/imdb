use hyper::header::{HeaderMap, ACCEPT_LANGUAGE, USER_AGENT};
use log::info;
use reqwest;
use url::Url;

use crate::consts;
use crate::error::Error;
use crate::language::Language;
use crate::models::Movie;
use crate::parser::top250;

/// Client to retrieve information from IMDb.

pub struct IMDb {
    /// Reqwest client
    client: reqwest::Client,
    /// Language
    lang: Option<String>,
    /// User-Agent
    ua: Option<String>,
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
    pub fn user_agent(&mut self, ua: String) -> &mut Self {
        self.ua = Some(ua);
        self
    }

    /// Makes HTTP requests.
    async fn get(&self, path: &str) -> Result<reqwest::Response, Error> {
        let base = Url::parse(consts::BASE_URL).unwrap();
        let url = base.join(path).unwrap();

        let mut headers = HeaderMap::new();
        if let Some(lang) = &self.lang {
            headers.insert(ACCEPT_LANGUAGE, lang.clone().parse().unwrap());
        }
        if let Some(ua) = &self.ua {
            headers.insert(USER_AGENT, ua.clone().parse().unwrap());
        }

        info!("Sending HTTP request for `{}`.", url);
        self.client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(Error::ReqwestError)
    }

    /// Get _Top 250 Movies_.
    pub async fn top250_movies(&self) -> Result<Vec<Movie>, Error> {
        let response = self.get("chart/top").await?;
        let html = response.text().await.map_err(Error::ReqwestError)?;
        let movies = top250::parse_top250_movies_html(&html);
        Ok(movies)
    }
}
