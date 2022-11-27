
pub enum Error {
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Builder {
}

impl Builder {
    pub fn new() -> Builder {
        Self { }
    }

    pub fn build(self) -> Tagger {
        Tagger { }
    }
}

pub struct Tagger {
}

impl Tagger {
    pub fn compute_tags<'t, 'raw>(&'t self, raw: &'raw [&'raw str]) -> Vec<&'t str> {
        todo!()
    }
}

