pub const LATEST_PATTERN: &str = include_str!("../dist/latest/rust-re2.txt");
pub const VERSION_15_0_PATTERN: &str = include_str!("../dist/emoji-15.0/rust-re2.txt");
pub const VERSION_14_0_PATTERN: &str = include_str!("../dist/emoji-14.0/rust-re2.txt");
pub const VERSION_13_1_PATTERN: &str = include_str!("../dist/emoji-13.1/rust-re2.txt");
pub const VERSION_13_0_PATTERN: &str = include_str!("../dist/emoji-13.0/rust-re2.txt");

lazy_static::lazy_static! {
    static ref LATEST_RE: regex::Regex  = regex::Regex::new(LATEST_PATTERN).unwrap();
}

pub fn is_emoji(s: &str) -> bool {
    LATEST_RE.find(s).is_some()
}
