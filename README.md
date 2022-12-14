# canister-rs

An API wrapper for the Canister API, in Rust.

**Note**: This is still a work in progress. Expect breaking
changes.

## Examples

`canister-rs` comes with both a normal, synchronous client,
as well as `async` support; whichever meets your needs!

### Using the `async` client

Make sure to enable the `async` feature while installing
the crate with cargo.

```rust
use canister_rs::Canister;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Always make sure to include a user-agent!
    let client = Canister::new("TheRealKeto/canister-rs");
    let data = client.search_canister("jailbreak/package/search", "siguza")
        .await?;

    println!("{:#?}", data);
    Ok(())
}
```

### Using the blocking/sync client

```rust
use canister_rs::Canister;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Always make sure to include a user-agent!
    let client = Canister::new("TheRealKeto/canister-rs");
    let data = client.search_canister("jailbreak/package/search", siguza);

    println!("{:#?}", data);
    Ok(())
}
```

## Morale

After writing `canister.py`, I decided I should put it to
good use by creating a CLI for it. Many know about this and
while I was able to do it, I did not like outcome.

I quickly realized that I wanted a more portable approach,
and my interest in Rust had strenghthen at the time, so
what better way to learn a language that to re-create
an old project.

This is a much better approach, particularly for creating
a CLI, as Rust is compiled down to an executable, rather
than being compiled to bytecode & interpreted; it's also
supported on every system imaginable.

## License

`canister-rs` is licensed under [BSD-3-Clause](LICENSE).
