#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContentType {
    Exam,
    ExamSolution,

    LectureSlides,
    TutorialNotes,
    SupplementaryNotes,

    Assignment,
    AssignmentSolution,

    LabManual,
    Project,

    #[default]
    Other,
}

impl From<&str> for ContentType {
    fn from(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "exam" => Self::Exam,
            "exam solution" => Self::ExamSolution,
            "lecture slides" => Self::LectureSlides,
            "tutorial notes" => Self::TutorialNotes,
            "supplementary notes" => Self::SupplementaryNotes,
            "assignment" => Self::Assignment,
            "assignment solution" => Self::AssignmentSolution,
            "lab manual" => Self::LabManual,
            "project" => Self::Project,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Content {
    title: String,
    content_type: ContentType,

    description: Option<String>,
    download_link: Option<String>,
}

pub struct ContentBuilder {
    title: String,
    content_type: ContentType,
    description: Option<String>,
    download_link: Option<String>,
}

impl ContentBuilder {
    pub fn new(title: String, content_type: ContentType) -> Self {
        ContentBuilder {
            title,
            content_type,
            description: None,
            download_link: None,
        }
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn download_link(&mut self, download_link: String) -> &mut Self {
        self.download_link = Some(download_link);
        self
    }

    pub fn build(self) -> Content {
        Content {
            title: self.title,
            content_type: self.content_type,
            description: self.description,
            download_link: self.download_link,
        }
    }
}
