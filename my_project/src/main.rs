use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{error::Error, fmt};

use serde_json::Value;

/// A common trait for anything that can fetch and persist a price.
trait Pricing {
    fn name(&self) -> &'static str;
    fn file_path(&self) -> &'static str;
    fn fetch_price(&self) -> Result<f64, FetchError>;

    fn save_to_file(&self, price: f64) -> Result<(), std::io::Error> {
        let path = self.file_path();
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;

        if is_empty(path) {
            writeln!(file, "timestamp_unix,asset,price_usd")?;
        }

        let ts = now_unix();
        writeln!(file, "{},{},{}", ts, self.name(), price)?;
        Ok(())
    }
}

fn is_empty(path: &str) -> bool {
    match std::fs::metadata(path) {
        Ok(m) => m.len() == 0,
        Err(_) => true,
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs()
}

struct Bitcoin;
struct Ethereum;
struct SP500;

#[derive(Debug)]
enum FetchError {
    Network(String),
    Parse(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Network(e) => write!(f, "network error: {}", e),
            FetchError::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}
impl Error for FetchError {}

/* ------------------------- Implementations ------------------------- */

impl Pricing for Bitcoin {
    fn name(&self) -> &'static str {
        "BTC"
    }
    fn file_path(&self) -> &'static str {
        "bitcoin_prices.csv"
    }
    fn fetch_price(&self) -> Result<f64, FetchError> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let resp = ureq::get(url)
            .header("User-Agent", "financial-data-fetcher/1.0")
            .call()
            .map_err(|e| FetchError::Network(format!("GET {}: {}", url, e)))?;

        let mut reader = resp.into_body().into_reader();
        let mut body = String::new();
        reader
            .read_to_string(&mut body)
            .map_err(|e| FetchError::Network(format!("read body: {}", e)))?;

        let v: Value = serde_json::from_str(&body)
            .map_err(|e| FetchError::Parse(format!("json: {} | body: {}", e, body)))?;

        v.get("bitcoin")
            .and_then(|b| b.get("usd"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| FetchError::Parse("missing field bitcoin.usd".into()))
    }
}

impl Pricing for Ethereum {
    fn name(&self) -> &'static str {
        "ETH"
    }
    fn file_path(&self) -> &'static str {
        "ethereum_prices.csv"
    }
    fn fetch_price(&self) -> Result<f64, FetchError> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let resp = ureq::get(url)
            .header("User-Agent", "financial-data-fetcher/1.0")
            .call()
            .map_err(|e| FetchError::Network(format!("GET {}: {}", url, e)))?;

        let mut reader = resp.into_body().into_reader();
        let mut body = String::new();
        reader
            .read_to_string(&mut body)
            .map_err(|e| FetchError::Network(format!("read body: {}", e)))?;

        let v: Value = serde_json::from_str(&body)
            .map_err(|e| FetchError::Parse(format!("json: {} | body: {}", e, body)))?;

        v.get("ethereum")
            .and_then(|b| b.get("usd"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| FetchError::Parse("missing field ethereum.usd".into()))
    }
}

impl Pricing for SP500 {
    fn name(&self) -> &'static str {
        "SP500"
    }
    fn file_path(&self) -> &'static str {
        "sp500_prices.csv"
    }
    fn fetch_price(&self) -> Result<f64, FetchError> {
        // Yahoo Finance chart endpoint for ^GSPC
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        let resp = ureq::get(url)
            .header("User-Agent", "financial-data-fetcher/1.0")
            .call()
            .map_err(|e| FetchError::Network(format!("GET {}: {}", url, e)))?;

        let mut reader = resp.into_body().into_reader();
        let mut body = String::new();
        reader
            .read_to_string(&mut body)
            .map_err(|e| FetchError::Network(format!("read body: {}", e)))?;

        let v: Value = serde_json::from_str(&body)
            .map_err(|e| FetchError::Parse(format!("json: {} | body: {}", e, body)))?;

        // chart.result[0].meta.regularMarketPrice
        v.get("chart")
            .and_then(|c| c.get("result"))
            .and_then(|arr| arr.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|obj| obj.get("meta"))
            .and_then(|m| m.get("regularMarketPrice"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| FetchError::Parse("missing chart.result[0].meta.regularMarketPrice".into()))
    }
}

/* ------------------------------ Main ------------------------------ */

fn main() {
    let assets: Vec<Box<dyn Pricing>> =
        vec![Box::new(Bitcoin), Box::new(Ethereum), Box::new(SP500)];

    println!("Starting Financial Data Fetcher (every 10 seconds). Press Ctrl+C to stop.");

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("[{}] failed to write CSV: {}", asset.name(), e);
                    } else {
                        println!("{} | {} | ${:.2}", now_unix(), asset.name(), price);
                    }
                }
                Err(e) => eprintln!("[{}] fetch error: {}", asset.name(), e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
