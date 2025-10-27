mod client;
use std::error::Error;

use client::AuthenticatedClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = AuthenticatedClient::new()?;

    client.authenticate("yassin.diab", "11223344Yd").await?;

    Ok(())
}
