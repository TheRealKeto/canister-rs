//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
use serde::Deserialize;
use reqwest::header::USER_AGENT;

#[derive(Debug, Deserialize)]
struct CanisterAPIResponse {
    status: String,
    date: String,
    message: Option<String>,
    data: Vec<CanisterPackage>
}

#[derive(Debug, Deserialize)]
struct CanisterPackage {
    identifier: String,
    name: Option<String>,
    description: String,
    author: Option<String>,
    maintainer: String,
    depiction: Option<String>,
    sileo_depiction: Option<String>,
    icon_url: Option<String>,
    repository: CanisterPackageRepo,
    color: Option<String>
}

#[derive(Debug, Deserialize)]
struct CanisterPackageRepo {
    uri: String
}

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
