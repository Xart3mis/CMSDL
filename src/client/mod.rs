use curl::easy::{Auth, Easy};

use super::{
    config::Credentials,
    parser::models::course::{Course, CoursesParser},
};

pub mod error;

const CMS_BASE_URL: &str = "https://cms.giu-uni.de/";
const CMS_HOME: &str = "apps/student/HomePageStn.aspx";
const CMS_COURSE_TEMPLATE: &str = "apps/student/CourseViewStn.aspx";

pub struct AuthenticatedClient {
    handle: Easy,
}

impl Default for AuthenticatedClient {
    fn default() -> AuthenticatedClient {
        Self::new()
    }
}

impl AuthenticatedClient {
    pub fn new() -> AuthenticatedClient {
        AuthenticatedClient {
            handle: Easy::new(),
        }
    }

    pub fn authenticate(&mut self, credentials: &Credentials) -> Result<(), curl::Error> {
        self.handle.http_auth(Auth::new().ntlm(true))?;

        self.handle.username(&credentials.username)?;
        self.handle.password(&credentials.password)?;

        Ok(())
    }

    pub fn get(&mut self, url: &str) -> Result<String, error::Error> {
        self.handle.url(url)?;

        let mut response_data = Vec::new();
        {
            let mut transfer = self.handle.transfer();
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }

        Ok(String::from_utf8(response_data)?)
    }
}

pub trait GetHtmlExt {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html, error::Error>;
}

impl GetHtmlExt for CoursesParser {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html, error::Error> {
        Ok(scraper::Html::parse_document(
            &client.get(format!("{}{}", CMS_BASE_URL, CMS_HOME,).as_str())?,
        ))
    }
}

impl GetHtmlExt for Course {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html, error::Error> {
        Ok(scraper::Html::parse_document(
            &client.get(
                format!(
                    "{}{}?id={}&sid={}",
                    CMS_BASE_URL, CMS_COURSE_TEMPLATE, self.course_id, self.season_id,
                )
                .trim(),
            )?,
        ))
    }
}
