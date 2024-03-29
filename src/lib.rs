//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
pub mod models;
use models::CanisterAPIResponse;

// Select request client based on selected feature
use reqwest::header;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;
#[cfg(feature = "blocking")]
use reqwest::blocking::Client;

#[derive(thiserror::Error, Debug)]
pub enum CanisterAPIError {
    #[error("Could not complete request")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Could not fetch Canister data")]
    BadRequest,
    #[error("404, Data not found")]
    NotFound,
    #[error("Unknown error: {0}")]
    UnknownError(String)
}

// Model of a Canister client
#[derive(Debug)]
pub struct Canister {
    client: Client,
    user_agent: String,
}

// Get the current version of the project
// Used within the user agent given to requests
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Base URL of all Canister API requests
const BASE_URL: &str = "https://api.canister.me/v2";

impl Default for Canister {
    fn default() -> Self {
        // Provide a default client & user agent
        Self {
            client: Client::new(),
            user_agent: format!("canister-rs/{}", VERSION)
        }
    }
}

impl Canister {
    // Creates a new instance of the client
    // A required user agent is already provided!
    pub fn new() -> Self {
        Self::default()
    }

    // Handle errors based on status codes
    // Bad requests are pretty terrible on v1
    fn map_response_err(&self, code: String) -> CanisterAPIError {
        match code.as_str() {
            "400 Bad Request" => CanisterAPIError::BadRequest,
            "404 Not Found" => CanisterAPIError::NotFound,
            _ => CanisterAPIError::UnknownError(code)
        }
    }

    #[maybe_async::maybe_async]
    pub async fn search_canister(
        &self, endpoint: &str, query: &str
    ) -> Result<CanisterAPIResponse, CanisterAPIError> {
        let request_url = format!("{}/{}", BASE_URL, endpoint);
        let response: CanisterAPIResponse = self.client
            .get(request_url)
            .header(header::USER_AGENT, &self.user_agent)
            .query(&[("q", query)])
            .send()
            .await?
            .json()
            .await?;

        match response.status.as_str() {
            "200 OK" => Ok(response),
            _ => Err(self.map_response_err(response.status))
        }
    }
}
