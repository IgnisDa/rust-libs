mod cli;

use avatars::{female_avatar, generate_seed, male_avatar, Mood as AvatarMood};
use cli::{Gender, Mood};
use color_eyre::Result;

pub fn run_command(seed_string: String, mood: Mood, gender: Gender) -> Result<String> {
    Ok("".to_string())
}

pub use cli::Cli;
