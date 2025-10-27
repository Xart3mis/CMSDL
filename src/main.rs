use base64::prelude::{BASE64_STANDARD, Engine};
use ntlmclient::OsVersion;
const EWS_URL: &str = "https://cms.giu-uni.de/";

async fn initialize_authed_client(username: &str, password: &str) -> reqwest::Client {
    let client = reqwest::Client::builder()
        .user_agent("CMSDL_Browserless.rs")
        .cookie_store(true)
        // .redirect(reqwest::redirect::Policy::none()) // Don't auto-follow redirects
        .build()
        .expect("failed to build client");

    let initial_resp = client
        .get(EWS_URL)
        .send()
        .await
        .expect("failed to send initial request");

    let auth_url = initial_resp.url().clone();

    let nego_flags = ntlmclient::Flags::from_bits(0x02898205).expect("Failed to construct flags");

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
    let nego_msg_bytes = nego_msg
        .to_bytes()
        .expect("failed to encode NTLM negotiation message");
    let nego_b64 = BASE64_STANDARD.encode(&nego_msg_bytes);

    dbg!(format!("NTLM {}", nego_b64));

    let resp = client
        .get(auth_url)
        .header("Authorization", format!("NTLM {}", nego_b64))
        .send()
        .await
        .expect("failed to send challenge request to Exchange");

    let challenge_header = resp
        .headers()
        .get("WWW-Authenticate")
        .expect("response missing challenge header");

    // we might have been redirected to a specialized authentication URL
    let auth_url = resp.url();

    let challenge_b64 = challenge_header
        .to_str()
        .expect("challenge header not a string")
        .split(" ")
        .nth(1)
        .expect("second chunk of challenge header missing");

    let challenge_bytes = BASE64_STANDARD
        .decode(challenge_b64)
        .expect("base64 decoding challenge message failed");

    let challenge = ntlmclient::Message::try_from(challenge_bytes.as_slice())
        .expect("decoding challenge message failed");

    let challenge_content = match challenge {
        ntlmclient::Message::Challenge(c) => c,
        other => panic!("wrong challenge message: {:?}", other),
    };

    dbg!(&challenge_content.flags);
    dbg!(format!(
        "Challenge flags: 0x{:08x}",
        challenge_content.flags.bits()
    ));

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
    let auth_msg = challenge_response.to_message(&creds, "WORKSTATION", challenge_content.flags);
    let auth_msg_bytes = auth_msg
        .to_bytes()
        .expect("failed to encode NTLM authentication message");
    let auth_b64 = BASE64_STANDARD.encode(&auth_msg_bytes);

    dbg!(format!("NTLM {}", auth_b64));

    let auth_response = client
        .get(auth_url.clone())
        .header("Authorization", format!("NTLM {}", auth_b64))
        .send()
        .await
        .expect("failed to send authentication request to Exchange");
    // .error_for_status()
    // .expect("error response to authentication message");

    dbg!(auth_response.status());
    dbg!(auth_response.headers());

    auth_response
        .error_for_status()
        .expect("error response to authentication message");

    // client
    //     .get(EWS_URL)
    //     .send()
    //     .await
    //     .expect("failed to send refresher request to Exchange")
    //     .error_for_status()
    //     .expect("error response to refresher message");

    client
}

#[tokio::main]
async fn main() {
    let client = initialize_authed_client("", "").await;
}
