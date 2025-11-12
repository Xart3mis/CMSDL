use anyhow::Context;

use super::{
    AuthenticatedClient, Content, ContentBuilder, Course, GetHtmlExt, Parsable, Regex, Selector,
    fix_html,
};

impl Parsable<Vec<Content>> for Course
where
    Course: GetHtmlExt,
{
    fn parse(&self, client: &mut AuthenticatedClient) -> anyhow::Result<Vec<Content>> {
        let document = self.get_html(client).context("Failed to get html")?;

        let content_selector =
            Selector::parse(".card.weeksdata .card.mb-4").expect("Failed to parse selector");

        let full_title_selector =
            Selector::parse(".card-body > div[id^=\"content\"]").expect("Failed to parse selector");

        let description_selector = Selector::parse(".card-body > div[id^=\"content\"] + div")
            .expect("Failed to parse selector");

        let title_selector = Selector::parse("strong").expect("Failed to parse selector");
        let link_selector = Selector::parse(".card-body a.btn.btn-primary.contentbtn#download")
            .expect("Failed to parse selector");

        let re = Regex::new(r"^\d+\s*-\s*(.+)$").context("Failed to create title regex")?;

        let mut content_list = Vec::new();

        for content_fr in document.select(&content_selector) {
            let full_title = content_fr
                .select(&full_title_selector)
                .nth(0)
                .context("Failed to select full content title")?;

            let title = fix_html(
                full_title
                    .select(&title_selector)
                    .nth(0)
                    .context("Failed to fetch title")?
                    .inner_html(),
            );

            let title = re
                .captures(&title)
                .context("Failed to capture regex in title")?
                .get(1)
                .context("Failed to get n1 regex capture")?;

            let mut builder = ContentBuilder::new(
                title.as_str().to_string(),
                full_title
                    .text()
                    .nth(1)
                    .context("Failed to get type")?
                    .trim()
                    .replacen("(", "", 1)
                    .trim()
                    .replacen(")", "", 1)
                    .trim()
                    .into(),
            );

            if let Some(description) = content_fr
                .select(&description_selector)
                .nth(0)
                .map(|v| v.text().nth(0).unwrap_or(""))
                && !description.is_empty()
            {
                builder.description(description.to_string());
            }

            if let Some(download_link_h) = content_fr
                .select(&link_selector)
                .nth(0)
                .map(|v| v.attr("href"))
                && let Some(download_link) = download_link_h
                && !download_link.is_empty()
            {
                builder.download_link(download_link.trim().to_string());
            }

            content_list.push(builder.build());
        }

        Ok(content_list)
    }
}
