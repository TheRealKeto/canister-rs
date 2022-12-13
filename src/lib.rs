//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
pub mod models;
use models::CanisterAPIResponse;

#[cfg(feature = "async")]
use reqwest::{header, Client};
#[cfg(not(feature = "async"))]
use reqwest::{header, blocking::Client};

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
    client: Client,
    user_agent: String,
}

// Base URL of all Canister API requests
// TODO: Migrate to v2 at some point in time
const BASE_URL: &str = "https://api.canister.me/v1/community";

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
            "400: Bad Request" => CanisterAPIError::BadRequest("Bad request"),
            _ => CanisterAPIError::BadRequest("Unknown error")
        }
    }

    #[cfg(not(feature = "async"))]
    pub fn search_canister(
        &self, endpoint: &str, query: &str
    ) -> Result<CanisterAPIResponse, CanisterAPIError> {
        let request_url = format!("{}/{}", BASE_URL, endpoint);
        let request = self.client
            .get(request_url)
            .header(header::USER_AGENT, &self.user_agent)
            .query(&[("query", query)])
            .build()?;

        let response: CanisterAPIResponse = self.client
            .execute(request)?
            .json()?;

        match response.status.as_str() {
            "Successful" => Ok(response),
            _ => Err(self.map_response_err(response.status))
        }
    }

    // Develop a differenet function for async support
    #[cfg(feature = "async")]
    pub async fn search_canister(
        &self, endpoint: &str, query: &str
    ) -> Result<CanisterAPIResponse, CanisterAPIError> {
        let request_url = format!("{}/{}", BASE_URL, endpoint);
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

        match response.status.as_str() {
            "Successful" => Ok(response),
            _ => Err(self.map_response_err(response.status))
        }
    }
}
