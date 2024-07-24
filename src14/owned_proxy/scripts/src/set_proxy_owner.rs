use clap::Parser;
use fuels::types::Address;
use proxy_script_utils::{get_proxy_instance, setup_signing_wallet, State};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Provider URL
    #[arg(long, default_value = "testnet.fuel.network")]
    provider_url: String,
    /// Signing key
    #[arg(short, long, required = true, env = "SIGNING_KEY")]
    signing_key: String,
    /// Proxy Contract Id
    #[arg(long, required = true)]
    proxy_contract_id: String,
    /// New Owner Id
    #[arg(short, long, required = true)]
    new_owner: String,
}

#[tokio::main]
async fn main() {
    println!("\n|||||||||||||||||||||||||||||||||\n-|- Setting a new proxy owner -|-\n|||||||||||||||||||||||||||||||||");

    let args = Args::parse();

    let signing_wallet = setup_signing_wallet(&args.provider_url, &args.signing_key).await;

    let proxy_contract = get_proxy_instance(&args.proxy_contract_id, signing_wallet);

    let current_owner = proxy_contract
        .methods()
        .proxy_owner()
        .simulate()
        .await
        .unwrap()
        .value;
    println!("\n - The current proxy owner: {:?}", current_owner);

    let new_owner = State::Initialized(
        Address::from_str(&args.new_owner)
            .expect("New owner Id could not be parsed")
            .into(),
    );

    println!(" - Proxy owner is being updated...");
    proxy_contract
        .methods()
        .set_proxy_owner(new_owner)
        .call()
        .await
        .unwrap();

    let new_owner = proxy_contract
        .methods()
        .proxy_owner()
        .simulate()
        .await
        .unwrap()
        .value;
    println!(" - The new proxy owner: {:?}\n", new_owner);
}
