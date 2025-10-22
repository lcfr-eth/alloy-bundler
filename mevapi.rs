/*
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::str::FromStr;
use url::Url;

use alloy::{
    hex, primitives::{b256, Bytes}, providers::{Provider, ProviderBuilder}, signers::local::PrivateKeySigner
};
//use alloy_mev::EthMevProviderExt;


use alloy::providers::RootProvider;        // or ProviderBuilder
use alloy_rpc_types_mev::{EthBundleHash, EthSendBundle, PrivateTransactionPreferences, EthSendPrivateTransaction};
use alloy::providers::ext::MevApi;   // trait with send_bundle / send_mev_bundle

use serde::Deserialize;
use serde_json::json; 

#[derive(Parser, Debug)]
#[command(
    name = "bundle-raw-tx-urls",
    about = "Send a raw signed tx as a 1-tx bundle to *your* builder URLs (targets next block by default)."
)]
struct Cli {
    /// L1 HTTP RPC URL (for block number).
    //#[arg(long, env = "ETH_HTTP_RPC")]
    //rpc_url: String,

    /// The raw signed tx hex (RLP) beginning with 0x…
    #[arg(long)]
    raw_tx: String,

    /// One or more builder URLs. Repeat the flag for multiple.
    /// Example: --builder-url https://relay1.example.org --builder-url https://relay2.example.org
    //#[arg(long = "builder-url", required = true)]
    //builder_urls: Vec<String>,

    /// Optional header-auth private key used for all supplied builder URLs (Flashbots-style).
    /// If omitted, builders will be added *without* header auth.
    #[arg(long, env = "BUNDLE_PRIVATE_KEY")]
    bundle_key: Option<String>,

    /// How many blocks ahead to target (default: 1 = next block).
    #[arg(long, default_value_t = 1u64)]
    block_offset: u64,

    /// Optional: bundle min timestamp (seconds since epoch).
    #[arg(long)]
    min_ts: Option<u64>,

    /// Optional: bundle max timestamp (seconds since epoch).
    #[arg(long)]
    max_ts: Option<u64>,

    /// Optional replacement UUID for cancel/replace semantics.
    #[arg(long)]
    replacement_uuid: Option<String>,

    /// Verbose logging.
    #[arg(long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    //dotenv().ok();
    let args = Cli::parse();

    // Provider (no tx wallet needed since we’re sending a *signed* raw tx)
    let provider = ProviderBuilder::new()
        .connect_http("https://virginia.builder.blockrazor.io".parse()?);
        

    let preferences = PrivateTransactionPreferences::default(); // TODO: set bundle preferences if needed

    // parse raw signed tx hex into Bytes
    let raw_tx: Bytes = {
        let s = args.raw_tx.trim();
        if !s.starts_with("0x") {
            return Err(anyhow!("--raw-tx must start with 0x"));
        }
        Bytes::from_str(s).context("Invalid --raw-tx hex")?
    };

    let bundle = EthSendBundle {
        txs: vec![raw_tx],
        ..EthSendBundle::default()
    };

    let x = provider.send_bundle(bundle).await?;


*/
