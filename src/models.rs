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
#[serde(rename_all = "camelCase")]
pub struct CanisterPackage {
    pub uuid: String,
    pub package: String,
    pub is_current: bool,
    pub is_pruned: bool,
    pub repository_tier: i32,
    pub repository_slug: String,
    pub price: String,
    pub version: String,
    pub architecture: String,
    pub filename: String,
    pub size: i32,
    pub sha256: Option<String>,
    pub name: Option<String>,
    pub description: String,
    pub author: Option<String>,
    pub maintainer: String,
    pub depiction: Option<String>,
    pub native_depiction: Option<String>,
    pub sileo_depiction: Option<String>,
    pub header: Option<String>,
    pub tint_color: Option<String>,
    pub icon: Option<String>,
    pub section: String,
    pub tags: Vec<String>,
    pub installed_size: i32,
    pub refs: CanisterPackageRefs
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackageRepo {
    pub uri: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanisterResponseRefs {
    pub next_page: Option<String>,
    pub previous_page: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackageRefs {
    pub meta: String,
    pub repo: String
}
