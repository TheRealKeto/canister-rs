//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
use canister_rs::Canister;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Make sure to always setup a user agent
    // let client = Canister::new("TheRealKeto/canister-rs");
    // let data = client.search_canister("packages/search", "siguza-utils")
        // .await?;
//
    // println!("{:#?}", data);
    // Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Always make sure to include a user-agent!
    let client = Canister::new("TheRealKeto/canister-rs");
    let data = client.search_canister("packages/search", "siguza-utils");

    println!("{:#?}", data);
    Ok(())
}
