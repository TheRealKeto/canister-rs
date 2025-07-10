//
// Copyright (c) 2025 Keto
// SPDX-License-Identifier: BSD-3-Clause
//
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CanisterAPIResponse {
    pub status: String,
    pub date: String,
    pub refs: CanisterResponseRefs,
    pub count: i32,
    pub data: Vec<CanisterPackage>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct CanisterPackage {
    pub id: String,
    pub package_id: String,
    pub latest_version: bool,
    pub visible: bool,
    pub quality: i32,
    pub repository_id: String,
    pub price: String,
    pub version: String,
    pub architecture: String,
    pub package_filename: String,
    pub package_size: i32,
    pub sha256_hash: String,
    pub name: Option<String>,
    pub description: String,
    pub author: String,
    pub maintainer: String,
    pub depiction: Option<String>,
    pub native_depiction: Option<String>,
    #[serde(rename(deserialize = "sileodepiction"))]
    pub sileo_depiction: Option<String>,
    pub header_url: Option<String>,
    pub tint_color: Option<String>,
    pub icon_url: Option<String>,
    pub section: Option<String>,
    pub tags: Option<Vec<String>>,
    pub installed_size: Option<i32>,
    pub search_vector: Option<String>,
    pub package: String,
    #[serde(rename(deserialize = "repositorytier"))]
    pub repository_tier: i32,
    pub rank: Option<i32>,
    pub repository: CanisterPackageRepo,
    pub refs: CanisterPackageRefs
}

#[derive(Debug, Deserialize)]
pub struct CanisterPackageRepo {
    pub id: String,
    pub uri: String,
    pub date: Option<String>,
    pub name: String,
    pub slug: String,
    pub tier: i32,
    pub suite: String,
    pub aliases: Option<Vec<String>>,
    pub quality: i32,
    pub version: String,
    pub visible: bool,
    pub sections: Vec<Option<String>>,
    pub bootstrap: bool,
    pub component: Option<String>,
    pub description: String,
    #[serde(rename(deserialize = "isBootstrap"))]
    pub is_bootstrap: bool,
    pub package_count: i32,
    pub search_vector: Option<String>,
    pub sileo_endpoint: Option<String>,
    pub origin_hostname: String,
    pub payment_gateway: Option<String>,
    pub origin_uses_https: bool,
    pub origin_last_updated: String,
    pub origin_release_hash: String,
    pub origin_release_path: String,
    pub origin_packages_hash: String,
    pub origin_packages_path: String,
    pub origin_has_in_release: bool,
    pub origin_has_release_gpg: bool,
    pub origin_supports_payment_v1: bool,
    pub origin_supports_payment_v2: bool
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
