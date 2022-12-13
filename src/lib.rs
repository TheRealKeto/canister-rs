//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
pub mod models;
use models::CanisterAPIResponse;

use reqwest::header;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum CanisterAPIError {
    #[error("Failed fetching data")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Bad request: {0}")]
    BadRequest(&'static str)
}

// Model of a Canister client
#[derive(Debug)]
pub struct Canister {
    user_agent: String,
    client: reqwest::Client,
}

impl Canister {
    // Creates a new instance of the client
    pub fn new(user_agent: &str) -> Canister {
        Canister {
            user_agent: user_agent.to_string(),
            client: reqwest::Client::new()
        }
    }

    // Error handling for failed request
    fn map_response_err(&self, code: String) -> CanisterAPIError {
        match code.as_str() {
            "400: Bad Request" => CanisterAPIError::BadRequest("Bad request"),
            _ => CanisterAPIError::BadRequest("Unknown error")
        }
    }

    pub async fn search_canister(
        &self, endpoint: &str, query: &str
    ) -> Result<CanisterAPIResponse, CanisterAPIError> {
        // Setup a base URL for the Canister API
        // then, append the given endpoint to it
        let base_url = "https://api.canister.me/v1/community";
        let request_url = format!("{}/{}", base_url, endpoint);

        // Make a request and get a response
        let request = self.client
            .get(request_url)
            .header(header::USER_AGENT, &self.user_agent)
            .query(&[("query", query)])
            .build()?;

        let response: CanisterAPIResponse = self.client
            .execute(request)
            .await?
            .json()
            .await?;

        // Safely parse the response and handle errors
        match response.status.as_str() {
            "Successful" => Ok(response),
            _ => Err(self.map_response_err(response.status))
        }
    }
}
