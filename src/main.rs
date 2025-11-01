mod client;
use std::collections::HashMap;

use client::{AuthenticatedClientBuilder, Credentials};

mod parser;
use parser::{Content, Course, CoursesParser, Parsable};

mod downloader;
use downloader::Download;

fn main() {
    let creds = Credentials::new("yassin.diab", "11223344Yd");

    let mut client_builder = AuthenticatedClientBuilder::new();
    client_builder.authenticate(&creds);

    let mut client = client_builder.build().unwrap();

    let courses = CoursesParser::new()
        .parse(&mut client)
        .expect("Failed to fetch & parse courses");

    let mut course_content: HashMap<Course, Vec<Content>> = HashMap::new();

    for course in courses {
        let content = course
            .parse(&mut client)
            .expect("Failed to fetch & parse courses");

        course_content.insert(course, content);
    }

    course_content.download(2, "Download", &creds);
}
