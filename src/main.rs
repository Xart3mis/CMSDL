pub mod client;
use client::AuthenticatedClient;

pub mod parser;
use parser::{Content, Course, CoursesParser, Parsable};

pub mod downloader;
use downloader::Download;

pub mod utils;
use utils::CourseFilter;

pub mod config;
use config::{Config, Credentials, DownloadOptions, GeneralOptions};

use clap::Parser;
use dialoguer::{MultiSelect, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};

use std::{collections::HashMap, error::Error, path::PathBuf, time::Duration};

pub const DEFAULT_MAX_CONCURRENCY: usize = 3;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let config: Config;

    let mut courses_to_dl = Vec::new();
    if let Some(courses) = args.courses {
        courses_to_dl = courses;
    }

    if let Some(username) = args.username
        && let Some(password) = args.password
        && let Some(save_path) = args.path
    {
        config = Config {
            credentials: Credentials::new(&username, &password),
            general_options: GeneralOptions::default(),
            download_options: DownloadOptions {
                max_concurrency: None,
                max_file_size: None,
                save_path,
            },
        };

        config.save()?;
    } else {
        config = Config::load()?;
    }

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(10));
    bar.set_style(ProgressStyle::with_template(
        "{spinner:.cyan.bold} {msg:.bold}",
    )?);

    bar.set_message("Authenticating Client...");

    std::thread::sleep(Duration::from_secs(1));

    let mut client = AuthenticatedClient::new();
    client.authenticate(&config.credentials)?;

    bar.finish_with_message(format!(
        "Successfully authenticated user: {}",
        config.credentials.username
    ));
    eprintln!("\n");

    bar.reset();
    bar.enable_steady_tick(Duration::from_millis(10));
    bar.set_message("Scraping Courses...");

    let fetched_courses: Vec<Course> = CoursesParser::new().parse(&mut client)?.deduplicate();

    bar.finish_with_message(format!("Got {} Courses.", fetched_courses.len()));
    eprintln!("\n");

    let mut courses = fetched_courses;
    if config.general_options.interactive_filtering {
        let selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select courses to download. (ESC or 'q' to download all)")
            .items_checked(
                courses
                    .iter()
                    .cloned()
                    .map(|x| (x, false))
                    .collect::<Vec<(Course, bool)>>(),
            )
            .interact_opt()?;

        if let Some(selection_idcs) = selection {
            courses = selection_idcs.iter().map(|&i| courses[i].clone()).collect();
        }
    }

    if !courses_to_dl.is_empty() {
        bar.reset();
        bar.enable_steady_tick(Duration::from_millis(10));
        bar.set_message("Filtering Courses...");

        if let Some(found) = courses.find_by_ids(&courses_to_dl) {
            courses = found;

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

        let content = course.parse(&mut client)?;

        let size = content.len();

        course_content.insert(course, content);

        bar.finish_with_message(format!("Got {} Files.", size));
        total_count += size;
        eprintln!();
    }

    eprintln!("\x1b[1mGot {} Total Files.\x1b[0m", total_count);

    course_content.download(config.download_options, &config.credentials)?;

    eprintln!("\x1b[1mFinished.\x1b[0m");

    Ok(())
}
