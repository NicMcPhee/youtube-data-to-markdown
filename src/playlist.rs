use core::fmt;
use std::{collections::HashMap, fs::File, io::Write, fmt::format};

use chrono::{DateTime, Local};
use phf::phf_map;
use serde::{Deserialize, Serialize};
use tera::{Tera, Context, Result, Value};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    kind: String,
    etag: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    url: String,
    width: usize,
    height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
    standard: Option<Thumbnail>,
    maxres: Option<Thumbnail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceId {
    kind: String,
    video_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    published_at: DateTime<Local>,
    title: String,
    description: String,
    thumbnails: Thumbnails,
    resource_id: ResourceId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    snippet: Snippet,
}

// We want to:
//  - Remove leading and trailing quotes
//  - Replace "\n" with return chars ('\n')
//  - Replace escaped double quotes ("\"") with just double quotes
//  - Replace matching parens in backticks with angle brackets
//    - Actually, I'm not doing this. There are probably about as
//      many parens that should stay parens as parens that should be
//      angle brackets. I think I'll just need to modify these by hand.
//      Sigh.
// This always returns `Ok`, so Clippy thinks we don't really need a
// `Result` type here. It's required by `tera`, though, so we'll just
// let it go.
#[allow(clippy::unnecessary_wraps)]
fn clean_text(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let text = value.to_string();
    // Strip off leading and trailing double quotes.
    let text = text.trim().trim_matches('"');

    let text = text.replace("\\n", "\n");
    let text = text.replace("\\\"", "\"");

    let val = tera::Value::from(text);
    Ok(val)
}

fn add_quotes(s: &str) -> String {
    "\"".to_owned() + s + "\""
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.md") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec![".html", ".sql"]);
        tera.register_filter("clean_text", clean_text);
        tera
    };
}

#[cfg(test)]
mod reading_templates_tests {
    use super::*;

    #[test]
    fn reads_templates() {
        let template_names = TEMPLATES.get_template_names();
        assert_eq!(1, template_names.count());
        let mut template_names = TEMPLATES.get_template_names();
        assert_eq!("article.md", template_names.next().unwrap());
    }
}

const RUSTLINGS_PLAYLIST: &str = "\"PLI9i5fpXEEc6g4tZJsnOPKVjnGkOCMKmm\"";
const ICE_REPOS_PLAYLIST: &str = "\"PLI9i5fpXEEc40_5gjSO--whmr_5Yp-aJN\"";
const ECHO_PLAYLIST: &str = "\"PLI9i5fpXEEc6GHl9wyZUWm9UwtO-1Qj7d\"";
const SEGMENTED_CLIENT_PLAYLIST: &str = "\"PLI9i5fpXEEc6_o2Xy0ozg_hrO4FgswkGG\"";
const RUST_GA_PLAYLIST: &str = "\"PLI9i5fpXEEc7E8W7wkWYuzXgvPAv8Emkl\"";

static SUBJECT_KEYWORD_MAP: phf::Map<&'static str, [&str; 3]> = phf_map! {
    "rustlings" => ["Rustlings", "exercises", "learn"],
    "ice-repos" => ["ice-repos", "archiv", "repositor"],
    "echo" => ["echo", "server", "thread"],
    "segmented" => ["segmented", "lab", "packet"],
    "rust-ga" => ["rust-ga", "population", "bitstring"],
};

static PLAYLIST_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "rustlings" => RUSTLINGS_PLAYLIST,
    "ice-repos" => ICE_REPOS_PLAYLIST,
    "echo" => ECHO_PLAYLIST,
    "segmented" => SEGMENTED_CLIENT_PLAYLIST,
    "rust-ga" => RUST_GA_PLAYLIST,
};

impl Video {
    pub fn title(&self) -> String {
        let mut result = "\"".to_string();
        result.push_str(&self.snippet.title);
        result.push('"');
        result
    }

    fn publication_date(&self) -> String {
        let date_string = self.snippet.published_at;
        date_string.format("%Y-%m-%d").to_string()
    }

    fn short_description(&self) -> &str {
        let full_description = &self.snippet.description;
        // Descriptions should never be empty.
        #[allow(clippy::unwrap_used)]
        full_description.split('\n').next().unwrap()
    }

    fn count_keywords(&self, keywords: &[&str; 3]) -> usize {
        let full_description = &self.snippet.description.to_lowercase();
        keywords.iter()
                .map(|keyword| {
                    full_description.matches(&keyword.to_lowercase()).count()
                })
                .sum::<usize>()
    }

    fn video_subject(&self) -> &str {
        // There should always be at least one keyword so unwrapping should be safe
        #[allow(clippy::unwrap_used)]
        SUBJECT_KEYWORD_MAP.entries()
            .map(|(label, kws)| (label, self.count_keywords(kws)))
            .max_by_key(|(_, count)| *count)
            .unwrap()
            .0
    }

    fn description(&self) -> &str {
        &self.snippet.description
    }

    pub fn filename(title: &str) -> String {
        lazy_static! {
            static ref EPISODE_REGEX: Regex = 
                // If this regex doesn't build, then we want everything to die.
                #[allow(clippy::unwrap_used)]
                Regex::new(r"[Ee]pisode (?P<ep_num>\d+)").unwrap();
        }
        // If we don't have "Episode XXX" in the title, I want to die so we know there's a problem.
        #[allow(clippy::unwrap_used)]
        let caps = EPISODE_REGEX.captures(title).unwrap();
        #[allow(clippy::unwrap_used)]
        let episode_number = caps.name("ep_num").unwrap().as_str();
        let result = format!("episode_{:0>4}.md", episode_number);
        result
    }

    fn make_context(&self) -> Context {
        let subject = self.video_subject();
        let title: &str = &self.title();
        let mut context = Context::new();
        context.insert("title", title);
        context.insert("date", &self.publication_date());
        context.insert("description", self.short_description());
        // `subject` better be a key in the `SUBJECT_MAP` or we have a problem
        #[allow(clippy::unwrap_used)]
        context.insert("subject", &add_quotes(subject));
        context.insert("code", &add_quotes(&self.snippet.resource_id.video_id));
        // `subject` better be a key in the `PLAYLIST_MAP` or we have a problem
        #[allow(clippy::unwrap_used)]
        context.insert("playlist_code", PLAYLIST_MAP.get(subject).unwrap());
        context.insert("body", self.description());
        context.insert("filename", &Self::filename(title));
        context
    }

    /// # Panics
    /// 
    /// This can panic in a host of ways yet to be discovered.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let context = self.make_context();
        // There better be an `article.md` template
        #[allow(clippy::unwrap_used)]
        TEMPLATES.render("article.md", &context).unwrap()
    }

    pub fn write_markdown_file(&self) {
        let markdown = self.to_markdown();
        let filename
            = "markdown_outputs/".to_owned() + &Video::filename(&self.title());
        let mut file = File::create(filename).unwrap();
        // If there's a problem writing the file I want to crash
        #[allow(clippy::unwrap_used)]
        file.write_all(markdown.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod context_tests {
    use crate::parsing::get_videos;

    use super::*;

    #[test]
    fn first_context() {
        let videos: Vec<Video> = get_videos("data/eps_page_1.json").unwrap();
        let first_video = &videos[0];
        let context = first_video.make_context();
        assert_eq!(&add_quotes(&first_video.snippet.title), context.get("title").unwrap());
        assert_eq!("2022-12-10", context.get("date").unwrap());
        assert_eq!("The second half of another really productive day!", context.get("description").unwrap());
        assert_eq!(&add_quotes("rust-ga"), context.get("subject").unwrap());
        assert_eq!(&add_quotes(&first_video.snippet.resource_id.video_id), context.get("code").unwrap());
        assert_eq!(&RUST_GA_PLAYLIST, context.get("playlist_code").unwrap());
        assert_eq!(&first_video.snippet.description, context.get("body").unwrap());
        assert_eq!("episode_61.md", context.get("filename").unwrap());
    }
}
