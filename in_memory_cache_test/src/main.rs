use cache_lib::{cached, Cache};
use rand::Rng;
use std::error::Error;
use std::time::Duration;
use tracing::info;
use tracing_subscriber;

struct RPC;

impl RPC {
    async fn get_balance(&self, address: &str) -> Result<u64, String> {
        // Simulating an API call with random success/failure
        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            let balance = rng.gen_range(100..10000);
            info!(
                "RPC call successful for address {}: balance {}",
                address, balance
            );
            Ok(balance)
        } else {
            info!("RPC call failed for address {}", address);
            Err("Error: RPC call failed".to_string())
        }
    }
}

struct Balances<'a> {
    cache: Cache<(&'a str,), u64>,
    rpc: RPC,
}

impl<'a> Balances<'a> {
    #[cached(cache_time = 10, cache_field_name = "cache")]
    async fn get_balance(&self, address: &'a str) -> Result<u64, String> {
        self.rpc.get_balance(address).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let balances = Balances {
        cache: Cache::new(),
        rpc: RPC,
    };

    for i in 0..15 {
        let result = balances.get_balance("123").await;
        info!("Iteration {}: Result = {:?}", i, result);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
