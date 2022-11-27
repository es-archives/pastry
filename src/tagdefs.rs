
use pulldown_cmark::{self as pdcm, Event, HeadingLevel};

#[derive(Debug, Clone)]
pub enum Error {
}

//pub type Result<T> = std::result::Result<T, Error>;

pub fn process(src: &str) -> Result<(), Error> {
    let events = pdcm::Parser::new_ext(src, pdcm::Options::all());


    Ok(())
}

/// Tags
///
/// Syntactically, tags are strings that satisfy certain rules.
///
/// A string is a tag if and only if it:
/// - Only contains Latin-1 codepoints from the following classes:
///     - alpha-numeric
///         - 0 thru 9 (hex: 30 thru 39)
///         - a thru z (hex: 61 thru 7a)
///     - spacer
///         - <space> - _ (hex: 20 2d 5f)
///     - symbol
///         - ! & ' ( ) . (hex: 21 26 27 28 29 2e)
/// - Starts and ends with an alpha-numeric codepoint
/// - Contains no spacer codepoint followed by another spacer codepoint
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

pub struct ParseTagError;
impl TryFrom<&str> for Tag {
    type Error = ParseTagError;
    fn try_from(s: &str) -> Result<Tag, Self::Error> {
        let s = s.trim().as_bytes();

        let chars_ok = s.iter()
            .all(|&ch| match ch {
                b'a'..=b'z' | b'0'..=b'9' => true,
                ch if b" -_!&'().".contains(&ch) => true,
                _ => false
            });

        for &ch in s.iter() {
            let ch = match ch {
            };
            if ch != SUBST || tag.last() != Some(&SUBST) {
                tag.push(ch);
            }
        }

        let tag_str = std::str::from_utf8(s).unwrap().trim_matches(SUBST as char);
        Ok(Tag(tag_str.to_owned()))
    }
}

struct TagSet {
    tags: std::collections::HashSet<Tag>,
}

