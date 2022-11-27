
mod meta;
mod tagdefs;
mod tagger;

use {
    meta::Meta,
    tagger::Tagger,
};

static TEST_MD: &str = include_str!("../test.md");
static TAGS_MD: &str = include_str!("../tags.md");

fn main() {
    let tag_defs = tagdefs::process(TAGS_MD).unwrap();
    //let tagger = Tagger::new(tag_defs);

    let meta = Meta::extract(TEST_MD);


    eprintln!("{:#?}", meta);
}

pub(crate) fn fnv(bytes: impl AsRef<[u8]>) -> u64 {
    let bytes = bytes.as_ref();
    let mut h = 0xcbf29ce484222325_u64;
    for &b in bytes {
        h = h ^ b as u64;
        h = h.wrapping_mul(0x100000001b3_u64);
    }
    h
}

