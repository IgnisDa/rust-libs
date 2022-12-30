mod cli;

pub fn run(options: &Cli) {
    dbg!(options);
    if options.list_countries {
        list_countries();
    }
}

pub fn list_countries() {}

pub use cli::Cli;
