mod cli;
mod mirrors;

struct CountryDetail {
    country: String,
    code: String,
    count: u8,
}

pub fn run(options: &Cli) {
    if options.list_countries {
        list_countries(&options.url);
    }
}

pub fn list_countries(url: &str) {
    //
}

pub use cli::Cli;
