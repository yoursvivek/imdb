# imdb

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

*imdb* is a Rust library to retrieve information from [IMDb].

This is a _work in progress_ and far from complete. For the time being you can check out the excellent python implementation [IMDbPY], which is complete and very well maintained. 


## Installation

Add following lines to your `Cargo.toml`:

```toml
[dependencies]
imdb = "0.0.1"
```

## Example

```rust
use imdb::IMDb;

imdb = IMDb::new();
top250movies = imdb.top250_movies().unwrap()
```

## License

This library is released under MIT License.

[IMDbPY]: https://github.com/alberanid/imdbpy/
[IMDb]: http://www.imdb.com/
