use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
            "other" | _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct CourseContentBuilder {
    total_weeks: Option<i32>,
    content_count: Option<i32>,

    content: Vec<Content>,
    grouped_content: Box<HashMap<ContentType, Content>>,
}

#[derive(Debug, Clone)]
pub struct CourseContent {
    total_weeks: i32,
    content_count: i32,

    content: Vec<Content>,
    grouped_content: Box<HashMap<ContentType, Content>>,
}

impl CourseContentBuilder {
    pub fn new() -> Self {
        CourseContentBuilder {
            total_weeks: None,
            content_count: None,
            content: Vec::new(),
            grouped_content: Box::new(HashMap::new()),
        }
    }

    pub fn total_weeks(&mut self, total_weeks: i32) -> &mut Self {
        self.total_weeks = Some(total_weeks);
        self
    }

    pub fn content_count(&mut self, content_count: i32) -> &mut Self {
        self.content_count = Some(content_count);
        self
    }

    pub fn add_content(&mut self, content: Content) {
        self.content.push(content.clone());
        self.grouped_content.insert(content.content_type, content);
    }

    pub fn build(&self) -> CourseContent {
        CourseContent {
            total_weeks: self.total_weeks.unwrap_or_default(),
            content_count: self.content_count.unwrap_or_default(),
            content: self.content.clone(),
            grouped_content: self.grouped_content.to_owned(),
        }
    }
}
