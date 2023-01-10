//! Internal utilities meant to be only used within this crate.
pub trait StrExt<'a> {
    /// Convert a string into an [`Option`](Option).
    fn into_option(self) -> Option<&'a str>;
}

impl<'a> StrExt<'a> for &'a str {
    fn into_option(self) -> Option<&'a str> {
        match self.is_empty() {
            true => None,
            false => Some(self),
        }
    }
}
