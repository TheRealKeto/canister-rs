//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
pub mod models;
use models::CanisterAPIResponse;

// Select request client based on selected feature
#[cfg(not(feature = "blocking"))]
use reqwest::{header, Client};
#[cfg(feature = "blocking")]
use reqwest::{header, blocking::Client};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum CanisterAPIError {
    #[error("Failed fetching data")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Bad request: {0}")]
    BadRequest(&'static str),
    #[error("Not found: {0}")]
    NotFound(&'static str)
}

// Model of a Canister client
#[derive(Debug)]
pub struct Canister {
    client: Client,
    user_agent: String,
}

// Base URL of all Canister API requests
const BASE_URL: &str = "https://api.canister.me/v2";

impl Canister {
    // Creates a new instance of the client
    pub fn new(user_agent: &str) -> Canister {
        Canister {
            client: Client::new(),
            user_agent: user_agent.to_string(),
        }
    }

    // Handle errors based on status codes
    // Bad requests are pretty terrible on v1
    fn map_response_err(&self, code: String) -> CanisterAPIError {
        match code.as_str() {
            "400 Bad Request" => CanisterAPIError::BadRequest("400: Bad request"),
            "404 Not Found" => CanisterAPIError::NotFound("404: Not Found"),
            _ => CanisterAPIError::BadRequest("Unknown error")
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

        match response.message.as_str() {
            "200 Successful" => Ok(response),
            _ => Err(self.map_response_err(response.message))
        }
    }
}
