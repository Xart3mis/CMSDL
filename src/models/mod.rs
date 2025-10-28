pub struct CourseBuilder {
    title: Option<String>,
    season: Option<String>,
    code: Option<String>,

    is_active: Option<bool>,

    course_id: i32,
    season_id: i32,
}

#[derive(Debug, Clone)]
pub struct Course {
    title: String,
    season: String,
    code: String,

    is_active: bool,

    course_id: i32,
    season_id: i32,
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

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn season(mut self, season: String) -> Self {
        self.season = Some(season);
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
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
