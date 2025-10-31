mod client;
use client::AuthenticatedClient;

mod parser;
use parser::{CoursesParser, Parsable};

fn main() {
    let mut client = AuthenticatedClient::new();

    client.authenticate("yassin.diab", "11223344Yd").unwrap();

    let courses = CoursesParser::new()
        .parse(&mut client)
        .expect("Failed to fetch & parse courses");

    dbg!(&courses);

    for course in courses {
        let content = course
            .parse(&mut client)
            .expect("Failed to fetch & parse courses");

        dbg!(content);
    }
}
