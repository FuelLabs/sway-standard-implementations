use clap::Parser;
use fuels::types::ContractId;
use proxy_script_utils::{get_proxy_instance, setup_signing_wallet};
use std::str::FromStr;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Provider URL
    #[arg(long, default_value = "testnet.fuel.network")]
    provider_url: String,
    /// Signing key
    #[arg(short, long, required = true)]
    signing_key: String,
    /// Proxy Contract Id
    #[arg(long, required = true)]
    proxy_contract_id: String,
    /// New Target Contract Id
    #[arg(short, long, required = true)]
    new_target_id: String,
}

#[tokio::main]
async fn main() {
    println!("\n||||||||||||||||||||||||||||||||||\n-|- Setting a new proxy target -|-\n||||||||||||||||||||||||||||||||||");

    let args = Args::parse();

    let signing_wallet = setup_signing_wallet(&args.provider_url, &args.signing_key).await;

    let proxy_contract = get_proxy_instance(&args.proxy_contract_id, signing_wallet);

    let current_target = proxy_contract
        .methods()
        .proxy_target()
        .simulate()
        .await
        .unwrap()
        .value;
    println!("\n - The current target contract ID: {:?}", current_target);

    let new_target =
        ContractId::from_str(&args.new_target_id).expect("New Target Id could not be parsed");

    println!(" - Proxy target is being updated...");
    proxy_contract
        .methods()
        .set_proxy_target(new_target)
        .call()
        .await
        .unwrap();

    let new_target = proxy_contract
        .methods()
        .proxy_target()
        .simulate()
        .await
        .unwrap()
        .value;
    println!(" - The new target contract ID: {:?}\n", new_target);
}
