use clap::Parser;
use fuels::{
    programs::contract::{Contract, LoadConfiguration, StorageConfiguration},
    types::{transaction::TxPolicies, Address, ContractId},
};
use proxy_script_utils::{setup_signing_wallet, ProxyContract, ProxyContractConfigurables, State};
use std::str::FromStr;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Provider URL
    #[arg(short, long, default_value = "testnet.fuel.network")]
    provider_url: String,
    /// Signing key
    #[arg(short, long, required = true)]
    signing_key: String,
    /// Initial target ContractId
    #[arg(long, required = true)]
    initial_target: String,
    /// Initial owner Id
    #[arg(long, required = true)]
    initial_owner: String,
}

#[tokio::main]
async fn main() {
    println!("\n|||||||||||||||||||||||||||||||||||||||||||||||||\n-|- Deploying and Initializing Proxy Contract -|-\n|||||||||||||||||||||||||||||||||||||||||||||||||");
    let args = Args::parse();

    let signing_wallet = setup_signing_wallet(&args.provider_url, &args.signing_key).await;

    // Deploy proxy with args as configurables
    let storage_configuration = StorageConfiguration::default()
        .add_slot_overrides_from_file(
            "../contract/out/release/src14_owned_proxy-storage_slots.json",
        )
        .unwrap();

    let configurables = ProxyContractConfigurables::default()
        .with_INITIAL_TARGET(Some(
            ContractId::from_str(&args.initial_target)
                .expect("Initial target ContractId could not be parsed"),
        ))
        .unwrap()
        .with_INITIAL_OWNER(State::Initialized(
            Address::from_str(&args.initial_owner)
                .expect("Initial owner Id could not be parsed")
                .into(),
        ))
        .unwrap();

    let configuration = LoadConfiguration::default()
        .with_storage_configuration(storage_configuration)
        .with_configurables(configurables);

    println!("\n - Deploying proxy contract...");
    let proxy_contract_id = Contract::load_from(
        "../contract/out/release/src14_owned_proxy.bin",
        configuration,
    )
    .unwrap()
    .deploy(&signing_wallet, TxPolicies::default())
    .await
    .unwrap();
    println!(
        " - Proxy Contract Deployed with ContractId: {}",
        ContractId::from(&proxy_contract_id)
    );

    // Initialize proxy
    println!(" - Initializing proxy contract...");
    ProxyContract::new(proxy_contract_id, signing_wallet)
        .methods()
        .initialize_proxy()
        .call()
        .await
        .unwrap();
    println!(" - Proxy Contract initialized\n");
}
