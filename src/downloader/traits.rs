use super::{Content, ContentType, Course, Credentials, File, HashMap, ProgressBar};

pub struct DownloadHandler {
    pub file: File,
    pub prefix: String,
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
    fn download(&self, max_concurrent: usize, base: &str, credentials: &'a Credentials);
}

pub type CourseContent = HashMap<Course, Vec<Content>>;
