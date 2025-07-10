# canister-rs

An API wrapper for [Canister](https://canister.me/), written in Rust.

Capable of supporting both sync and async requests, the project
should be capable of handling whatever meets your needs.

**Warning**: There are no front-facing docs, as this is still a
work in progress. Expect breaking changes.

## Usage

To start, add the crate to your project using Cargo. You might want
to consider using the `--git` flag when adding the crate.

    cargo add canister-rs

By default, only async requests will be allowed when adding the
crate to your project. Should you need a sync implementation,
simply enable the `blocking` feature when adding the crate.

    cargo add canister-rs --features blocking


### Examples

Once added, the project can be imported like any crate. The example
below should provide a general idea on how to get started.

```rust
use canister::{Canister, CanisterAPIError};

// Remove this macro when using sync implementation
#[tokio::main]
async fn main() -> Result<(), CanisterAPIError> {
    // Look for packages matching the query, "aemulo"
    // Always make sure to include a user agent!
    let client = Canister::new();
    let data = client.search_canister("jailbreak/package/search", "aemulo")
        .await;

    println!("{data:#?}");
    Ok(())
}
```

Keep in mind that the sync and async implementations are identical,
apart from the `async/await` keywords.

## License

`canister-rs` is licensed under [BSD-3-Clause](LICENSE).
