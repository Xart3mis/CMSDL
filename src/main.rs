mod client;
use client::{AuthenticatedClientBuilder, Credentials};

mod parser;
use parser::{Content, Course, CoursesParser, Parsable};

mod downloader;
use downloader::Download;

mod utils;
use utils::{CourseFilter, is_valid_path};

use clap::Parser;
use dialoguer::{Input, Password, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};

use std::{collections::HashMap, path::PathBuf, time::Duration};

const MAX_CONCURRENT_DOWNLOADS: usize = 4;

/// CLI app to download & sync content from GIU CMS.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your GIU account username.
    #[arg(short, long)]
    username: Option<String>,

    /// Your GIU account pasword.
    #[arg(short, long)]
    password: Option<String>,

    /// Where all downloaded content is saved.
    #[arg(long)]
    path: Option<PathBuf>,

    /// Course IDs to download (downloads all if not specified) [e.g: --courses=34,2488]
    #[arg(long, value_delimiter = ',')]
    courses: Option<Vec<i32>>,
}

fn main() {
    let args = Args::parse();

    let username: String;
    let password: String;
    let save_to: PathBuf;

    let mut courses_to_dl = Vec::new();
    if let Some(courses) = args.courses {
        courses_to_dl = courses;
    }

    if let Some(username_) = args.username
        && let Some(password_) = args.password
        && let Some(save_to_) = args.path
    {
        username = username_;
        password = password_;
        save_to = save_to_;
    } else {
        username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Username")
            .interact_text()
            .unwrap();

        password = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.chars().count() >= 8 {
                    Ok(())
                } else {
                    Err("Password must be longer than 8 characters long.")
                }
            })
            .interact()
            .unwrap();

        save_to = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Save Downloads To")
            .validate_with(|input: &String| -> Result<(), &str> {
                if is_valid_path(input) {
                    Ok(())
                } else {
                    Err("Invalid path")
                }
            })
            .interact_text()
            .unwrap()
            .into();
    }

    let creds = Credentials::new(&username.to_lowercase(), &password);

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(10));
    bar.set_style(ProgressStyle::with_template("{spinner:.cyan.bold} {msg:.bold}").unwrap());

    bar.set_message("Authenticating Client...");

    std::thread::sleep(Duration::from_secs(1));

    let mut client_builder = AuthenticatedClientBuilder::new();
    client_builder.authenticate(&creds);

    let mut client = client_builder.build().unwrap();

    bar.finish_with_message(format!(
        "Successfully authenticated user: {}",
        creds.username
    ));
    eprintln!("\n");

    bar.reset();
    bar.enable_steady_tick(Duration::from_millis(10));
    bar.set_message("Scraping Courses...");

    let fetched_courses: Vec<Course> = CoursesParser::new()
        .parse(&mut client)
        .expect("Failed to fetch & parse courses")
        .deduplicate();

    bar.finish_with_message(format!("Got {} Courses.", fetched_courses.len()));
    eprintln!("\n");

    let mut courses = fetched_courses;
    if !courses_to_dl.is_empty() {
        dbg!(&courses_to_dl);

        bar.reset();
        bar.enable_steady_tick(Duration::from_millis(10));
        bar.set_message("Filtering Courses...");

        if let Some(found) = courses.find_by_ids(&courses_to_dl) {
            courses = found;

            dbg!(&courses);
            bar.finish_with_message(format!("Filtered {} Courses.", courses.len()));
            eprintln!("\n");
        } else {
            bar.finish_with_message("Failed to Filter Courses. Downloading all...");
            eprintln!("\n");
        }
    }

    let mut course_content: HashMap<Course, Vec<Content>> = HashMap::new();

    let mut total_count = 0;
    for course in courses {
        bar.reset();
        bar.enable_steady_tick(Duration::from_millis(10));
        bar.set_message(format!("Scraping {} Content...", course.title));

        let content = course
            .parse(&mut client)
            .expect("Failed to fetch & parse courses");

        let size = content.len();

        course_content.insert(course, content);

        bar.finish_with_message(format!("Got {} Files.", size));
        total_count += size;
        eprintln!();
    }

    eprintln!("\x1b[1mGot {} Total Files.\x1b[0m", total_count);

    course_content
        .download(MAX_CONCURRENT_DOWNLOADS, &save_to.to_string_lossy(), &creds)
        .unwrap();

    eprintln!("\x1b[1mFinished.\x1b[0m");
}
