use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, Table};
use mirrors::{count_countries, get_mirror_status};
use std::path::Path;
mod cli;
mod mirrors;

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
    let mut sorted = vec![];
    for (country, count) in counts {
        sorted.push((country, count));
    }
    sorted.sort_by(|c1, c2| c1.0.code.cmp(&c2.0.code));
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec!["Country", "Code", "Count"]);
    for (country, count) in sorted {
        table.add_row(vec![
            Cell::new(country.kind.to_string()),
            Cell::new(country.code.to_string()),
            Cell::new(count.to_string()).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{}", table);
}

pub use cli::Cli;
