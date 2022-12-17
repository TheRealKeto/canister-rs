//
// Copyright (c) 2022, Keto (TheRealKeto)
// SPDX-License-Identifier: BSD-3-Clause
//
use canister_rs::Canister;

fn main() -> Result<(), reqwest::Error> {
    // Always make sure to include a user-agent!
    let client = Canister::new("TheRealKeto/canister-rs");
    let data = client.search_canister("jailbreak/package/search", "siguza");

    println!("{:#?}", data);
    Ok(())
}
