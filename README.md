# In-Memory Cache System with Blockchain RPC Example

This project implements a helper in-memory cache system for Rust, allowing easy caching of any function results for a specific short duration using a custom attribute macro. It includes an example of caching blockchain RPC calls to get address balances.

## Project Structure

The project is organized as a workspace with two crates:

1. `cache_macro`: Contains the proc macro implementation.
2. `src`: Contains the cache implementation, re-exports the macro, and includes examples.

## Features

- Asynchronous cache implementation using `tokio`
- Tracing support for debugging and logging
- Generic cache that works with any hashable key and clonable value
- Time-to-live (TTL) support for cached items
- Custom attribute macro for easy cache usage
- Example of caching blockchain RPC calls

## Requirements

- Rust 1.56 or later
- Cargo

## Usage

1. Add the `cache_lib` crate to your project's dependencies.
2. Import the `Cache` struct and `cached` macro from `cache_lib`.
3. Use the `#[cached]` attribute macro on your async functions:

```rust
use cache_lib::{Cache, cached};

struct CachedBlockchainRPC {
    cache: Cache<String, Result<u64, Box<dyn Error>>>,
    rpc: BlockchainRPC,
}

impl CachedBlockchainRPC {
    #[cached(cache_time = 10, cache_field_name = "cache")]
    async fn get_balance(&self, address: String) -> Result<u64, Box<dyn Error>> {
        self.rpc.get_balance(&address).await
    }
}
```

## Testing the Blockchain RPC Example

1. Run the example:
   ```
   cargo run 
   ```

You should see that the balance is fetched from the RPC for the first call, and then cached for subsequent calls within the cache time.

## TODO

- Implement blockchain RPC call
- Support env file for RPC URL
- Add proper error handling (consider using `anyhow` or `thiserror` crates)
- Implement cache eviction policies (e.g., LRU) to manage memory usage
- Add more comprehensive tests and benchmarks

## Extra Tools Used

- `tokio`: Asynchronous runtime for Rust
- `tracing`: Logging and instrumentation for Rust programs
- `syn`, `quote`, and `proc-macro2`: For implementing the custom attribute macro
- `reqwest`: HTTP client for making RPC calls
- `serde` and `serde_json`: For JSON serialization and deserialization
- `dotenv`: For loading environment variables

## License

This project is licensed under the MIT License.