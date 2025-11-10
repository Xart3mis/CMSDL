use super::{
    Content, ContentType, Course, Credentials, DownloadOptions, File, HashMap, ProgressBar, Result,
};

pub struct DownloadHandler {
    pub max_file_size: Option<usize>,
    pub downloaded_size: usize,
    pub file: File,

    pub scroll_offset: usize,
    pub pb: ProgressBar,
    pub prefix: String,

    pub error_msg: Option<String>,
}

pub struct DownloadableItem {
    pub course: Course,
    pub title: String,
    pub content_type: ContentType,

    pub description: Option<String>,
    pub download_link: Option<String>,
}

pub trait Download<'a> {
    fn download(
        &self,
        download_options: DownloadOptions,
        credentials: &'a Credentials,
    ) -> Result<()>;
}

pub type CourseContent = HashMap<Course, Vec<Content>>;
