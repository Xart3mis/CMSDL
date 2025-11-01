pub struct CourseBuilder {
    title: Option<String>,
    season: Option<String>,
    code: Option<String>,

    is_active: Option<bool>,

    course_id: i32,
    season_id: i32,
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq)]
pub struct Course {
    pub title: String,
    pub season: String,
    pub code: String,

    pub is_active: bool,

    pub course_id: i32,
    pub season_id: i32,
}

pub type Courses = Vec<Course>;

pub struct CoursesParser;

impl CoursesParser {
    pub fn new() -> Self {
        Self
    }
}

impl CourseBuilder {
    pub fn new(course_id: i32, season_id: i32) -> Self {
        CourseBuilder {
            course_id,
            season_id,
            title: None,
            season: None,
            code: None,
            is_active: None,
        }
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn season(&mut self, season: String) -> &mut Self {
        self.season = Some(season);
        self
    }

    pub fn code(&mut self, code: String) -> &mut Self {
        self.code = Some(code);
        self
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn build(self) -> Course {
        Course {
            title: self.title.unwrap_or_default(),
            season: self.season.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
            is_active: self.is_active.unwrap_or(true),
            course_id: self.course_id,
            season_id: self.season_id,
        }
    }
}
