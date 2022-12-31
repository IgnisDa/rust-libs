mod cli;

use avatars::{female_avatar, generate_seed, male_avatar, Mood as AvatarMood};
use cli::{Gender, Mood};
use color_eyre::Result;

pub fn run_command(seed_string: String, mood: Mood, gender: Gender) -> Result<String> {
    let seed = generate_seed(&seed_string);
    let mood = match mood {
        Mood::Happy => AvatarMood::Happy,
        Mood::Sad => AvatarMood::Sad,
        Mood::Surprised => AvatarMood::Surprised,
    };
    Ok(match gender {
        Gender::Female => female_avatar(seed, &mood),
        Gender::Male => male_avatar(seed, &mood),
    })
}

pub use cli::Cli;
