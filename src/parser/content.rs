use super::{
    AuthenticatedClient, Content, ContentBuilder, Course, GetHtmlExt, Parsable, Regex, Selector,
    error::ParseError, fix_html,
};

impl Parsable<Vec<Content>> for Course
where
    Course: GetHtmlExt,
{
    fn parse(&self, client: &mut AuthenticatedClient) -> Result<Vec<Content>, ParseError> {
        let document = self.get_html(client)?;

        let content_selector = Selector::parse(".card.weeksdata .card.mb-4")?;

        let full_title_selector = Selector::parse(".card-body > div[id^=\"content\"]")?;

        let description_selector = Selector::parse(".card-body > div[id^=\"content\"] + div")?;

        let title_selector = Selector::parse("strong").expect("Failed to parse selector");
        let link_selector = Selector::parse(".card-body a.btn.btn-primary.contentbtn#download")?;

        let re = Regex::new(r"^\d+\s*-\s*(.+)$")?;

        let mut content_list = Vec::new();

        for content_fr in document.select(&content_selector) {
            if let Some(full_title) = content_fr.select(&full_title_selector).nth(0)
                && let Some(raw_title) = full_title.select(&title_selector).nth(0)
            {
                let title = fix_html(raw_title.inner_html());

                if let Some(captures) = re.captures(&title)
                    && let Some(title) = captures.get(1)
                    && let Some(raw_type) = full_title.text().nth(1)
                {
                    let mut builder = ContentBuilder::new(
                        title.as_str().to_string(),
                        raw_type
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
            }
        }

        Ok(content_list)
    }
}
