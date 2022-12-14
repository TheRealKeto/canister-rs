//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CanisterAPIResponse {
    pub message: String,
    pub date: String,
    pub refs: CanisterResponseRefs,
    pub count: i32,
    pub data: Vec<CanisterPackage>
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackage {
    pub package: String,
    #[serde(rename = "isCurrent")]
    pub is_current: bool,
    #[serde(rename = "repositorySlug")]
    pub repository_slug: String,
    pub price: String,
    pub tier: i32,
    pub version: String,
    pub architecture: String,
    pub filename: String,
    pub size: String,
    #[serde(rename = "sha256")]
    pub sha_256: String,
    pub name: Option<String>,
    pub description: String,
    pub author: Option<String>,
    pub maintainer: String,
    pub depiction: Option<String>,
    #[serde(rename = "sileoDepiction")]
    pub sileo_depiction: Option<String>,
    pub header: Option<String>,
    #[serde(rename = "tintColor")]
    pub color: Option<String>,
    pub icon: Option<String>,
    pub section: String,
    pub tag: Vec<String>,
    #[serde(rename = "installedSize")]
    pub installed_size: String,
    pub refs: CanisterPackageRefs
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackageRepo {
    pub uri: String
}

#[derive(Debug, Deserialize)]
pub struct CanisterResponseRefs {
    #[serde(rename = "nextPage")]
    pub next_page: Option<String>,
    #[serde(rename = "previousPage")]
    pub previous_page: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackageRefs {
    pub meta: String,
    pub repo: String
}
