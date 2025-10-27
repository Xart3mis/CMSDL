use anyhow::{Ok, Result, anyhow};
use std::rc::Rc;

use base64::prelude::{BASE64_STANDARD, Engine};
use ntlmclient::OsVersion;

const HOME: &str = "https://cms.giu-uni.de/apps/student/HomePageStn.aspx";

pub struct AuthenticatedClient {
    client: Rc<reqwest::Client>,
}

impl AuthenticatedClient {
    pub fn new() -> Result<AuthenticatedClient> {
        Ok(AuthenticatedClient {
            client: Rc::new(
                reqwest::Client::builder()
                    .cookie_store(true)
                    .build()?,
            ),
        })
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<()> {
        let initial_resp = self
            .client
            .get(HOME)
            .send()
            .await?;
        let auth_url = initial_resp.url().clone();

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
            .get(auth_url)
            .header("Authorization", format!("NTLM {}", nego_b64))
            .send()
            .await?;

        let challenge_header = resp
            .headers()
            .get("WWW-Authenticate")
            .ok_or(anyhow!("challenge header not present"))?;

        // we might have been redirected to a specialized authentication URL
        let auth_url = resp.url();

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

        let target_info_bytes: Vec<u8> = challenge_content
            .target_information
            .iter()
            .flat_map(|ie| ie.to_bytes())
            .collect();

        let creds = ntlmclient::Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
            domain: "GIUAS".to_owned(),
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

        let auth_response = self
            .client
            .get(auth_url.clone())
            .header("Authorization", format!("NTLM {}", auth_b64))
            .send()
            .await?;

        auth_response.error_for_status()?;

        Ok(())
    }
}
