use anyhow::Result;
use curl::easy::{Auth, Easy};

use crate::parser::models::course::{Course, CoursesParser};

pub struct AuthenticatedClient {
    handle: Box<Easy>,
}

impl AuthenticatedClient {
    pub fn new() -> AuthenticatedClient {
        AuthenticatedClient {
            handle: Box::new(Easy::new()),
        }
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<()> {
        self.handle.http_auth(Auth::new().ntlm(true))?;

        self.handle.username(username)?;
        self.handle.password(password)?;

        Ok(())
    }

    pub fn get(&mut self, url: &str) -> Result<String> {
        self.handle.url(url)?;

        let mut response_data = Vec::new();
        {
            let mut transfer = self.handle.transfer();
            transfer
                .write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform()?;
        }

        Ok(String::from_utf8(response_data)?)
    }
}

pub trait GetHtmlExt {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html>;
}

impl GetHtmlExt for CoursesParser {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html> {
        Ok(scraper::Html::parse_document(&client.get(
            "https://cms.giu-uni.de/apps/student/HomePageStn.aspx",
        )?))
    }
}

impl GetHtmlExt for Course {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html> {
        Ok(scraper::Html::parse_document(
            &client.get(
                format!(
                    "https://cms.giu-uni.de/apps/student/CourseViewStn.aspx?id={}&sid={}",
                    self.course_id, self.season_id
                )
                .trim(),
            )?,
        ))
    }
}
