mod client;
use client::AuthenticatedClient;

mod parser;
use parser::{Courses, CoursesExt, CoursesParser, Parsable, Content};

fn main() {
    let mut client = AuthenticatedClient::new();

    client.authenticate("yassin.diab", "11223344Yd").unwrap();

    let courses = CoursesParser::new()
        .parse(&mut client)
        .expect("Failed to fetch & parse courses");

    dbg!(&courses);

    for course in courses {
        course.parse(&mut client).expect("Failed to fetch & parse courses");
    }
}
