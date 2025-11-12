use regex::Regex;
use scraper::Selector;

pub mod content;
pub mod course;
pub mod error;

use html_escape::decode_html_entities;
fn fix_html(s: String) -> String {
    decode_html_entities(&s).replace("\u{a0}", "")
}

pub trait Parsable<O> {
    fn parse(&self, client: &mut AuthenticatedClient) -> Result<O, error::ParseError>;
}

pub mod models;

use super::client::{AuthenticatedClient, GetHtmlExt};
use models::{content::ContentBuilder, course::CourseBuilder};

pub use models::content::{Content, ContentType};
pub use models::course::{Course, Courses, CoursesParser};
