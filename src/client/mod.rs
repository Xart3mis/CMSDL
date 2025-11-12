use anyhow::{Context, Result};
use curl::easy::{Auth, Easy};

use super::{
    config::Credentials,
    parser::models::course::{Course, CoursesParser},
};

const CMS_BASE_URL: &str = "https://cms.giu-uni.de/";
const CMS_HOME: &str = "apps/student/HomePageStn.aspx";
const CMS_COURSE_TEMPLATE: &str = "apps/student/CourseViewStn.aspx";

pub struct AuthenticatedClient {
    handle: Easy,
}

pub struct AuthenticatedClientBuilder<'a> {
    credentials: Option<&'a Credentials>,
}

impl<'a> AuthenticatedClientBuilder<'a> {
    pub fn new() -> Self {
        Self { credentials: None }
    }

    pub fn authenticate(&mut self, credentials: &'a Credentials) -> &mut Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn build(self) -> Result<AuthenticatedClient> {
        let mut client = AuthenticatedClient::new();

        if let Some(credentials) = self.credentials {
            client.authenticate(credentials)?;
        }

        Ok(client)
    }
}

impl AuthenticatedClient {
    pub fn new() -> AuthenticatedClient {
        AuthenticatedClient {
            handle: Easy::new(),
        }
    }

    pub fn authenticate(&mut self, credentials: &Credentials) -> Result<()> {
        self.handle.http_auth(Auth::new().ntlm(true))?;

        self.handle.username(&credentials.username)?;
        self.handle.password(&credentials.password)?;

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
                .context("Failed to set write function")?;
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
        Ok(scraper::Html::parse_document(
            &client.get(format!("{}{}", CMS_BASE_URL, CMS_HOME,).as_str())?,
        ))
    }
}

impl GetHtmlExt for Course {
    fn get_html(&self, client: &mut AuthenticatedClient) -> Result<scraper::Html> {
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
