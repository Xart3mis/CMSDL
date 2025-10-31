use anyhow::Result;
use regex::Regex;
use scraper::Selector;

pub mod content;
pub mod course;

pub trait Parsable<O> {
    fn parse(&self, client: &mut AuthenticatedClient) -> Result<O>;
}

pub mod models;

use super::client::{AuthenticatedClient, GetHtmlExt};
use models::{content::ContentBuilder, course::CourseBuilder};

pub use models::content::{Content, ContentType};
pub use models::course::{Course, Courses, CoursesExt, CoursesParser};
