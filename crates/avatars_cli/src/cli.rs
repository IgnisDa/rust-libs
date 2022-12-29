use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum Mood {
    Happy,
    Sad,
    Surprised,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Parser, Debug)]
#[command(
    about,
    author,
    version,
    propagate_version = true,
    next_line_help = false,
    disable_help_subcommand = true,
    after_long_help = "This tool can be used to generate random svg avatar images from an input string."
)]
pub struct Cli {
    /// The seed string
    pub seed_string: String,

    /// The mood of the avatar
    #[arg(short, long, required = true)]
    pub mood: Mood,

    /// The gender of the avatar
    #[arg(short, long, required = true)]
    pub gender: Gender,
}
