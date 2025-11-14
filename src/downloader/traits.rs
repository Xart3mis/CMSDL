use super::{
    Content, ContentType, Course, Credentials, DownloadOptions, File, HashMap, ProgressBar, error,
};

pub(crate) struct DownloadHandler {
    pub(crate) max_file_size: Option<usize>,
    pub(crate) downloaded_size: usize,
    pub(crate) file: File,

    pub(crate) scroll_offset: usize,
    pub(crate) pb: ProgressBar,
    pub(crate) prefix: String,

    pub(crate) error_msg: Option<String>,
}

pub(crate) struct DownloadableItem {
    pub(crate) course: Course,
    pub(crate) title: String,
    pub(crate) content_type: ContentType,

    pub(crate) description: Option<String>,
    pub(crate) download_link: Option<String>,
}

pub trait Download<'a> {
    fn download(
        &self,
        download_options: DownloadOptions,
        credentials: &'a Credentials,
    ) -> Result<(), error::Error>;
}

pub type CourseContent = HashMap<Course, Vec<Content>>;
