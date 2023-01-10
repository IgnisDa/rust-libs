use mirrors::{count_countries, get_mirror_status};
use std::path::Path;
mod cli;
mod mirrors;

struct CountryDetail {
    country: String,
    code: String,
    count: u8,
}

pub async fn run(options: &Cli) {
    if options.list_countries {
        list_countries(&options.url).await;
    }
}

pub async fn list_countries(url: &str) {
    let status = get_mirror_status(10, 10, url, &Path::new(""))
        .await
        .unwrap();
    let counts = count_countries(&status.urls).await;
    for (country, count) in counts {
        println!("{} has {} mirrors", country.kind.to_string(), count);
    }
}

pub use cli::Cli;
