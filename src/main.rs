use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde::Deserialize;
use serde_json::{json};
use std::str::FromStr;
use url::Url;

use alloy::{
    primitives::{B256, Bytes},
    providers::{Provider, ProviderBuilder},
};
use alloy_rpc_types_mev::EthBundleHash;

#[derive(Parser, Debug)]
#[command(
    name = "Bundler",
    about = "Send a raw signed tx as a 1-tx bundle to your builder (tolerant response parsing)."
)]
struct Cli {
    /// Builder URL (JSON-RPC endpoint of your relay/builder)
    #[arg(long, default_value = "https://virginia.builder.blockrazor.io")]
    builder_url: String,

    /// Raw signed tx (RLP hex starting with 0x…)
    #[arg(long)]
    raw_tx: String,

    /// Target next block = current + offset (only used if --rpc-url is provided)
    #[arg(long, default_value_t = 1u64)]
    block_offset: u64,

    /// Optional min/max timestamps for the bundle
    #[arg(long)]
    min_ts: Option<u64>,
    #[arg(long)]
    max_ts: Option<u64>,

    /// Optional replacement UUID
    #[arg(long)]
    replacement_uuid: Option<String>,

    /// JSON-RPC method name used by your relay. Common: eth_sendBundle or mev_sendBundle
    #[arg(long, default_value = "eth_sendBundle")]
    method: String,

    /// Verbose logs
    #[arg(long)]
    verbose: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BundleHashEither {
    Plain(String),
    Obj { bundle_hash: B256 },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let builder_url = Url::from_str(&args.builder_url)
        .with_context(|| format!("Invalid builder URL: {}", args.builder_url))?;

    let raw_tx: Bytes = {
        let s = args.raw_tx.trim();
        if !s.starts_with("0x") {
            return Err(anyhow!("--raw-tx must start with 0x"));
        }
        Bytes::from_str(s).context("Invalid --raw-tx hex")?
    };

    if args.verbose {
        eprintln!("[*] Builder:        {}", builder_url);
        eprintln!("[*] Method:         {}", args.method);
        eprintln!("[*] Raw tx bytes:   {}", raw_tx.len());
    }

    let mut params = json!({
        "txs": [args.raw_tx.trim()],
    });

    if let Some(bn) = Some(args.block_offset) {
        params["blockNumber"] = json!(bn);
    }
    if let Some(v) = args.min_ts {
        params["minTimestamp"] = json!(v);
    }
    if let Some(v) = args.max_ts {
        params["maxTimestamp"] = json!(v);
    }
    if let Some(v) = args.replacement_uuid.clone() {
        params["replacementUuid"] = json!(v);
    }

    let builder = ProviderBuilder::new()
        .connect_http(builder_url.as_str().parse()?);

    let res: BundleHashEither = builder
        .client()
        .request(args.method, [params])
        .await
        .map_err(|e| anyhow!("relay request failed: {e}"))?;

    let bundle_hash = match res {
        BundleHashEither::Plain(s) => s
            .parse::<B256>()
            .context("relay returned a string, but it wasn't a valid 0x<64-hex>")?,
        BundleHashEither::Obj { bundle_hash } => bundle_hash,
    };
    let out = EthBundleHash { bundle_hash };

    println!("ok bundle_hash={:#x}", out.bundle_hash);
    Ok(())
}
