use super::{
    AuthenticatedClient, CourseBuilder, Courses, CoursesParser, GetHtmlExt, Parsable, Regex,
    Selector, fix_html,
};

impl Parsable<Courses> for CoursesParser
where
    CoursesParser: GetHtmlExt,
{
    fn parse(&self, client: &mut AuthenticatedClient) -> anyhow::Result<Courses> {
        let document = self.get_html(client).expect("Failed to get html");

        let course_selector = Selector::parse("td").expect("Failed to parse Selector");
        let courses_selector = Selector::parse(
            "#ContentPlaceHolderright_ContentPlaceHoldercontent_GridViewcourses > tbody > tr",
        )
        .expect("Failed to parse Selector");

        let re = Regex::new(r"\(\|(?P<code>[A-Z0-9]+)\|\)\s+(?P<name>[^()]+)").unwrap();

        let mut courses_list = Vec::new();
        for courses in document.select(&courses_selector).skip(1) {
            let mut course = courses.select(&course_selector).skip(1);

            let title = course.next();
            let active = course.next();
            let season = course.next();

            let course_id = course.next();
            let season_id = course.next();

            if let Some(course_id) = course_id
                && let Some(season_id) = season_id
            {
                let mut course_builder = CourseBuilder::new(
                    course_id.inner_html().parse()?,
                    season_id.inner_html().parse()?,
                );

                if let Some(title) = title
                    && let Some(caps) = re.captures(&fix_html(title.inner_html()))
                {
                    course_builder.code(caps["code"].trim().to_string());
                    course_builder.title(caps["name"].trim().to_string());
                }

                if let Some(active) = active {
                    course_builder.is_active(active.inner_html().to_lowercase().eq("active"));
                }

                if let Some(season) = season {
                    course_builder.season(season.inner_html());
                }

                courses_list.push(course_builder.build());
            }
        }

        Ok(courses_list)
    }
}
