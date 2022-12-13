//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
use reqwest::header::USER_AGENT;

mod models;
use models::CanisterAPIResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup a base URL for the Canister API
    // then, include a valid endpoint inside
    let base_url = "https://api.canister.me/v1/community";
    let request_url = format!("{}/{}", base_url, "packages/search");

    // Start setting up the API request
    // then, retrieve the response
    let client = reqwest::Client::new();
    let request = client
        .get(request_url)
        .header(USER_AGENT, "TheRealKeto/canister-rs")
        .query(&[("query", "siguza-utils")])
        .send()
        .await?;

    let resp = request.json::<CanisterAPIResponse>().await?;
    println!("{:#?}", resp);
    Ok(())
}
