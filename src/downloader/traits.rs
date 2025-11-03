use super::{Content, ContentType, Course, Credentials, File, HashMap, ProgressBar, Result};

pub struct DownloadHandler {
    pub file: File,
    pub prefix: String,
    pub scroll_offset: usize,
    pub pb: ProgressBar,
}

pub struct DownloadableItem {
    pub course: Course,
    pub title: String,
    pub content_type: ContentType,

    pub description: Option<String>,
    pub download_link: Option<String>,
}

pub trait Download<'a> {
    fn download(&self, max_concurrent: usize, base: &str, credentials: &'a Credentials) ->Result<()>;
}

pub type CourseContent = HashMap<Course, Vec<Content>>;
