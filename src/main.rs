use curl::easy::{Auth, Easy};
use std::str;

fn main() {
    let mut easy = Easy::new();

    easy.url("https://cms.giu-uni.de/apps/student/HomePageStn.aspx").expect("easy.url failed");

    easy.http_auth(Auth::new().ntlm(true))
        .expect("failed to set up auth");

    easy.username("yassin.diab").unwrap();
    easy.password("11223344Yd").unwrap();

    let mut response_data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let body = String::from_utf8(response_data).unwrap();

    println!("{}", body.len());
    println!("{}", &body);
}

