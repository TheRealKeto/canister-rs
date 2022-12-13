//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CanisterAPIResponse {
    pub status: String,
    pub date: String,
    pub message: Option<String>,
    pub data: Vec<CanisterPackage>
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackage {
    pub identifier: String,
    pub name: Option<String>,
    pub description: String,
    pub section: Option<String>,
    pub author: Option<String>,
    pub maintainer: String,
    pub depiction: Option<String>,
    #[serde(rename = "nativeDepiction")]
    pub sileo_depiction: Option<String>,
    #[serde(rename = "packageIcon")]
    pub icon_url: Option<String>,
    pub version: Option<String>,
    pub repository: CanisterPackageRepo,
    pub color: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackageRepo {
    pub uri: String
}
