use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub use anyhow::Result;

use curl::{
    easy::{Auth, Easy2, Handler, WriteError},
    multi::{Easy2Handle, Multi},
};

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

mod traits;
use traits::{CourseContent, DownloadHandler, DownloadableItem};

use crate::client::Credentials;
use crate::parser::{Content, ContentType, Course};

pub use traits::Download;

impl Handler for DownloadHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.file.write_all(data).map_err(|_| WriteError::Pause)?;
        Ok(data.len())
    }

    fn progress(&mut self, dltotal: f64, dlnow: f64, _: f64, _: f64) -> bool {
        if dltotal > 0.0 {
            self.pb.set_length(dltotal as u64);
            self.pb.set_position(dlnow as u64);
        } else {
            self.pb.set_message("unknown size");
        }
        true
    }
}

impl DownloadableItem {
    fn new(course: Course, content: Content) -> Self {
        Self {
            course,
            title: content.title,
            content_type: content.content_type,
            description: content.description,
            download_link: content.download_link,
        }
    }

    fn path(&self, base: &str) -> Option<PathBuf> {
        if let Some(download_link) = &self.download_link {
            return Some(
                Path::new(
                    format!(
                        "{}/{}/{}/{}.{}",
                        base,
                        self.course.title,
                        self.content_type,
                        self.title,
                        download_link.trim().rsplit_once(".")?.1
                    )
                    .trim(),
                )
                .to_owned(),
            );
        }

        None
    }
}

impl<'a> Download<'a> for CourseContent {
    fn download(
        &self,
        max_concurrent: usize,
        base: &str,
        credentials: &'a Credentials,
    ) -> Result<()> {
        let sp = ProgressBar::new_spinner();

        sp.set_style(ProgressStyle::with_template(
            "{spinner:.cyan.bold} {msg:.bold}",
        )?);
        sp.enable_steady_tick(Duration::from_millis(15));

        sp.set_message("Starting Downloads...");

        thread::sleep(Duration::from_secs(2));

        let mp = Arc::new(MultiProgress::new());

        let style =
            ProgressStyle::with_template("{prefix:.dim} [{bar:30.magenta/black}] {percent:>3}%")?
                .progress_chars("█░ ");

        let mut multi = Multi::new();
        multi.pipelining(true, true)?;

        let mut queue: VecDeque<DownloadableItem> = self
            .iter()
            .flat_map(|(course, contents)| {
                contents
                    .iter()
                    .map(move |content| DownloadableItem::new(course.clone(), content.clone()))
            })
            .filter(|x| {
                x.content_type != ContentType::VoD && x.path(base).is_some_and(|y| !y.exists())
            })
            .collect();

        sp.finish();

        let mut handles: HashMap<usize, Easy2Handle<DownloadHandler>> = HashMap::new();
        let mut next_token = 0;
        // helper to start next queued download
        let start_next = |multi: &mut Multi,
                          queue: &mut VecDeque<DownloadableItem>,
                          handles: &mut HashMap<usize, Easy2Handle<DownloadHandler>>,
                          token: &mut usize|
         -> Result<bool> {
            if let Some(item) = queue.pop_front() {
                if let Some(download_link) = &item.download_link
                    && let Some(filename) = item.path(base)
                {
                    if let Some(parent) = filename.parent() {
                        std::fs::create_dir_all(parent)?;
                    }

                    let file = File::create(&filename)?;

                    let pre = format!(" {} -| {} ", item.course.title, item.title);

                    let pb = mp.add(ProgressBar::new(0));
                    pb.set_prefix(format_fixed_prefix(*token, pre.trim(), 60));
                    pb.set_style(style.clone());

                    let mut easy = Easy2::new(DownloadHandler {
                        file,
                        prefix: pre,
                        scroll_offset: 0,
                        pb: pb.clone(),
                    });

                    easy.http_auth(Auth::new().ntlm(true))?;
                    easy.username(&credentials.username)?;
                    easy.password(&credentials.password)?;

                    let url = format!("https://cms.giu-uni.de{}", download_link);

                    easy.url(&url)?;
                    easy.follow_location(true)?;
                    easy.progress(true)?;
                    easy.tcp_keepalive(true)?;

                    let mut handle = multi.add2(easy)?;
                    handle.set_token(*token)?;
                    handles.insert(*token, handle);
                    *token += 1;

                    return Ok(true);
                } else if let Some(description) = item.description {
                    let filename = format!("{}/{}/{}.txt", base, item.course.title, item.title);

                    if let Some(parent) = std::path::Path::new(&filename).parent() {
                        std::fs::create_dir_all(parent)?;
                    }

                    let mut file = File::create(&filename)?;
                    file.write_all(description.as_bytes())
                        .expect("Failed to write description to file");

                    return Ok(false);
                }
            }

            Ok(false)
        };

        // fill up initial concurrent slots
        for _ in 0..max_concurrent.min(queue.len()) {
            if !start_next(&mut multi, &mut queue, &mut handles, &mut next_token)? {
                continue;
            }
        }

        let mut last_scroll = Instant::now();
        let scroll_interval = Duration::from_millis(150);

        while !queue.is_empty() || !handles.is_empty() {
            multi.perform()?;

            if last_scroll.elapsed() >= scroll_interval {
                for (tk, h) in handles.iter_mut() {
                    let referance = h.get_mut();
                    let offset = &mut referance.scroll_offset;
                    let scrolled = scroll_prefix(*tk, &referance.prefix, 60, offset);
                    *offset += 1;
                    referance.pb.set_prefix(scrolled);
                }

                last_scroll = Instant::now();
            }

            let mut finished_tokens = Vec::new();
            multi.messages(|msg| {
                let token = msg.token().expect("failed to get token");
                let handle = handles
                    .get_mut(&token)
                    .expect("the download value should exist in the HashMap");

                match msg
                    .result_for2(handle)
                    .expect("token mismatch with the `EasyHandle`")
                {
                    Ok(()) => {
                        let http_status = handle
                            .response_code()
                            .expect("HTTP request finished without status code");

                        handle.get_ref().pb.finish_with_message(format!(
                            "R: Transfer succeeded (Status: {})",
                            http_status,
                        ));
                    }
                    Err(error) => {
                        handle
                            .get_ref()
                            .pb
                            .finish_with_message(format!("E: {}", error));
                    }
                }

                finished_tokens.push(token);
            });

            for token in finished_tokens {
                handles.remove(&token);
                start_next(&mut multi, &mut queue, &mut handles, &mut next_token)?;
            }

            if !handles.is_empty() {
                let timeout = multi.get_timeout()?;

                match timeout {
                    Some(duration) if duration == Duration::ZERO => continue,
                    Some(duration) => multi.wait(&mut [], duration)?,
                    None => multi.wait(&mut [], Duration::from_millis(100))?,
                };
            }
        }

        Ok(())
    }
}

fn scroll_prefix(token: usize, text: &str, width: usize, offset: &mut usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    if len == 0 {
        return format!("[{:04}] {}", token, " ".repeat(width));
    }

    let current_offset = *offset % len;
    let visible: String = chars
        .iter()
        .cycle()
        .skip(current_offset)
        .take(width)
        .collect();

    // Pad if shorter than width
    let visible = if visible.chars().count() < width {
        format!("{}{}", visible, " ".repeat(width - visible.chars().count()))
    } else {
        visible
    };

    format!("[{:04}] {}", token, visible)
}

fn format_fixed_prefix(token: usize, prefix: &str, width: usize) -> String {
    let mut out: String = prefix.chars().take(width).collect();
    if out.chars().count() < width {
        out.push_str(&" ".repeat(width - out.chars().count()));
    }

    format!("[{:04}] {}", token, out)
}
