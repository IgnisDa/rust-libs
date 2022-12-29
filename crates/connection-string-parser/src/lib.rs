mod cli;

use cli::Part;
use color_eyre::{eyre::eyre, Result};
use url::Url;

pub fn run_command(url: String, part: Part, fail_silently: bool) -> Result<String> {
    let parsed = Url::parse(&url)?;
    let resp = match part {
        Part::Scheme => Some(parsed.scheme().to_string()),
        Part::Host => parsed.host_str().map(String::from),
        Part::Port => parsed.port().map(|p| p.to_string()),
        Part::User => Some(parsed.username().to_string()),
        Part::Password => parsed.password().map(String::from),
        Part::Path => parsed
            .path_segments()
            .and_then(|mut p| p.next().map(String::from)),
    };
    if let Some(resp) = resp {
        Ok(resp)
    } else if fail_silently {
        Ok(String::new())
    } else {
        Err(eyre!("No value found for part {:?}", part))
    }
}

pub use cli::Cli;
