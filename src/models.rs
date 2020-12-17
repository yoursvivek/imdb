//! Data Models for `imdb`.
//!
//! These are various data models used across `imdb`. They are also publically available.
//!
//! Models also implement serialization and deserialization routines for [serde]. To use them
//! enable `serde-impls` feature.
//!
//! [serde]: http://serde.rs

use std::fmt;

/// Struct for Title ID
///
/// IMDb title `id`s are numerics represented as zero padded seven digits prefixed with `tt`.

#[cfg_attr(feature = "serde-impls", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TitleID(pub u32);

impl fmt::Display for TitleID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tt{:08}", self.0)
    }
}

/// Struct for Movie
///
/// Although IMDb represents all types of `title`s as one class, we will keep
/// movies on their own.

#[cfg_attr(feature = "serde-impls", derive(Serialize, Deserialize))]
#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Movie {
    /// ID
    #[allow(unused_mut)]
    pub id: TitleID,
    /// Title
    #[allow(unused_mut)]
    pub title: String,
    /// Year of release
    #[allow(unused_mut)]
    pub year: u16,
    /// Rating on IMDb
    #[allow(unused_mut)]
    pub rating: f32,
    /// Number of votes on IMDb
    #[allow(unused_mut)]
    pub votes: u32,
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}
