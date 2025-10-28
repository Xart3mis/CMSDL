use anyhow::{Ok, Result, anyhow};
use std::sync::Arc;

use reqwest::{
    Url,
    cookie::{CookieStore, Jar},
    header::{self, HeaderValue},
};
use scraper::Html;

use base64::prelude::{BASE64_STANDARD, Engine};
use ntlmclient::OsVersion;

const HOME: &str = "https://cms.giu-uni.de/apps/student/HomePageStn.aspx";

pub struct AuthenticatedClient {
    client: Arc<reqwest::Client>,
    jar: Arc<Jar>,

    final_token: String,

    username: String,
    password: String,
}

impl AuthenticatedClient {
    pub fn new(username: &str, password: &str) -> Result<AuthenticatedClient> {
        let jar = Arc::new(Jar::default());

        let mut headers = header::HeaderMap::new();

        headers.append(header::CONNECTION, HeaderValue::from_static("Keep-Alive"));
        headers.append(header::CONNECTION, HeaderValue::from_static("close"));
        headers.append(header::HOST, HeaderValue::from_static("cms.giu-uni.de"));
        headers.append(
            header::UPGRADE_INSECURE_REQUESTS,
            HeaderValue::from_static("1"),
        );
        headers.append(
            header::USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0",
            ),
        );

        Ok(AuthenticatedClient {
            client: Arc::new(
                reqwest::Client::builder()
                    // .redirect(reqwest::redirect::Policy::limited(10))
                    .default_headers(headers)
                    .http1_only()
                    .cookie_provider(Arc::clone(&jar))
                    .pool_max_idle_per_host(0)
                    // .tcp_keepalive(std::time::Duration::from_secs(30))
                    .build()?,
            ),
            jar,
            username: username.to_owned(),
            password: password.to_owned(),
            final_token: String::new(),
        })
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        // let initial_resp = self.client.get(HOME).send().await?;
        // let auth_url = initial_resp.url().clone();
        //
        // dbg!(initial_resp);
        // dbg!(auth_url.clone());

        let nego_flags = ntlmclient::Flags::from_bits(0x02898205)
            .ok_or(anyhow!("failed to construct negotiation flags"))?;

        let nego_msg = ntlmclient::Message::Negotiate(ntlmclient::NegotiateMessage {
            flags: nego_flags,
            supplied_domain: String::new(),
            supplied_workstation: String::new(),
            os_version: OsVersion {
                major_version: 10,
                build_number: 19041,
                ..Default::default()
            },
        });

        let nego_msg_bytes = nego_msg.to_bytes()?;
        let nego_b64 = BASE64_STANDARD.encode(&nego_msg_bytes);

        let resp = self
            .client
            // .get(auth_url)
            .get(HOME)
            .header("Authorization", format!("NTLM {}", nego_b64))
            .send()
            .await?;

        dbg!(&resp);

        let challenge_header = resp
            .headers()
            .get("WWW-Authenticate")
            .ok_or(anyhow!("challenge header not present"))?;

        let challenge_b64 = challenge_header
            .to_str()
            .expect("challenge header not a string")
            .split(" ")
            .nth(1)
            .ok_or(anyhow!("second chunk of challenge header missing"))?;

        let challenge_bytes = BASE64_STANDARD.decode(challenge_b64)?;

        let challenge = ntlmclient::Message::try_from(challenge_bytes.as_slice())?;

        let challenge_content = match challenge {
            ntlmclient::Message::Challenge(c) => c,
            other => panic!("wrong challenge message: {:?}", other),
        };

        dbg!(&challenge_content.flags);
        dbg!(&challenge_content.target_information);

        let target_info_bytes: Vec<u8> = challenge_content
            .target_information
            .iter()
            .flat_map(|ie| ie.to_bytes())
            .collect();

        let creds = ntlmclient::Credentials {
            username: self.username.to_owned(),
            password: self.password.to_owned(),
            // domain: "GIUAS".to_owned(),
            domain: "".to_owned(),
        };

        let challenge_response = ntlmclient::respond_challenge_ntlm_v2(
            challenge_content.challenge,
            &target_info_bytes,
            ntlmclient::get_ntlm_time(),
            &creds,
        );

        // assemble the packet
        let auth_msg =
            challenge_response.to_message(&creds, "WORKSTATION", challenge_content.flags);
        let auth_msg_bytes = auth_msg.to_bytes()?;
        let auth_b64 = BASE64_STANDARD.encode(&auth_msg_bytes);

        self.final_token = format!("NTLM {}", auth_b64);

        dbg!(self.final_token.len());
        dbg!(self.final_token.starts_with("NTLM "));

        let r = self
            .client
            .get(resp.url().clone())
            .header("Authorization", format!("NTLM {}", auth_b64))
            .send()
            .await?;
        // .error_for_status()?;

        dbg!(r.headers().get("WWW-Authenticate"));

        dbg!(&r);

        dbg!(r.headers());
        dbg!(self.jar.cookies(&Url::parse(HOME)?));

        let cookie_str = self.jar.cookies(&Url::parse(HOME)?).unwrap().clone();

        self.jar.add_cookie_str(
            cookie_str.to_str()?,
            &Url::parse("https://cms.giu-uni.de/")?,
        );

        Ok(())
    }

    pub async fn get_html_home(&mut self) -> Result<Html> {
        self.get_html(HOME).await
    }

    pub async fn get_html(&mut self, url: &str) -> Result<Html> {
        Ok(Html::parse_document(
            &self
                .client
                .get(url)
                .header("Authorization", &self.final_token)
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?,
        ))
    }
}
