
use {
    std::collections::HashSet,
    pulldown_cmark::{self as pdcm, Tag, Event, HeadingLevel},
};

pub use pdcm::CowStr;

#[derive(Debug)]
pub struct Meta<'md> {
    pub title: CowStr<'md>,
    pub authors: Vec<CowStr<'md>>,
    pub sources: Vec<CowStr<'md>>,
    pub tags: HashSet<String>,
    pub body: &'md str,
}

impl Meta<'_> {
    pub fn extract<'md>(md: &'md str) -> Meta<'md> {
        let mut title = None;
        let mut authors = Vec::new();
        let mut tags = HashSet::new();
        let mut sources = Vec::new();

        let mut events = pdcm::Parser::<'md, 'md>::new_ext(md, pdcm::Options::all());
        let post_break = loop {
            let Some(event) = events.next()
                else {panic!("file ends unexpectedly")};

            match event {
                Event::Start(tag) => match tag {
                    // # Title
                    Tag::Heading(HeadingLevel::H1, _, _) => {
                        if title.is_some() { panic!("title already set"); }
                        let Some(Event::Text(text)) = events.next()
                            else {panic!("titles must be plain text")};
                        title = Some(text);
                        let Some(Event::End(_)) = events.next()
                            else {panic!("titles must be plain text")};
                    },

                    // ## (various)
                    Tag::Heading(HeadingLevel::H2, _, _) => {
                        let Some(Event::Text(section)) = events.next()
                            else {panic!("expected metadata section label")};
                        let Some(Event::End(_)) = events.next()
                            else {panic!("metadata section labels must be plain text")};

                        // TODO refactor
                        match section.to_ascii_lowercase().as_str() {
                            "tags" => parse_section(&mut events,
                                |item| parse_tag_string(&mut tags, item.as_ref()),
                            ),

                            "sources" => parse_section(&mut events, |item| sources.push(item)),
                            "authors" => parse_section(&mut events, |item| authors.push(item)),

                            _ => { }
                        }
                    },

                    _ => { }
                },

                // ---
                Event::Rule => {
                    let mut iter = events.into_offset_iter();
                    let Some((_, range)) = iter.next()
                        else {panic!("file ends unexpectedly")};
                    break range.start;
                },

                _ => { }
            }
        };

        Meta {
            title: title.unwrap(),
            authors,
            sources,
            tags,
            body: &md[post_break..],
        }
    }
}

type Events<'a> = pdcm::Parser<'a, 'a>;

fn parse_tag_string<'a>(set: &mut HashSet<String>, text: &str) {
    set.extend(
        text.split(",")
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(Into::into)
    );
}

fn parse_section<'a>(events: &mut Events<'a>, mut f: impl FnMut(CowStr<'a>)) {
    let Some(Event::Start(Tag::List(_))) = events.next()
        else {panic!("meta section must contain single list only")};

    while let Some(Event::Start(Tag::Item)) = events.next() {
        // TODO accept more than just text
        let Some(Event::Text(text)) = events.next()
            else {panic!("each meta list item must be plain text")};
        f(text);
        let Some(Event::End(Tag::Item)) = events.next()
            else {panic!("each meta list item must be plain text")};
    }
}

