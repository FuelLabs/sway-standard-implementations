use clap::Parser;
use fuels::{
    programs::contract::{Contract, LoadConfiguration, StorageConfiguration},
    types::Address,
};
use proxy_script_utils::{ProxyContractConfigurables, State};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Initial target `ContractId`
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

    // Deploy proxy with args as configurables
    let storage_configuration = StorageConfiguration::default()
        .add_slot_overrides_from_file(
            "../contract/out/release/src14_owned_proxy-storage_slots.json",
        )
        .unwrap();

    let configurables = ProxyContractConfigurables::default()
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
    let proxy_contract = Contract::load_from(
        "../contract/out/release/src14_owned_proxy.bin",
        configuration,
    )
    .unwrap();

    let proxy_contract_id = proxy_contract.contract_id();
    let proxy_contract_bytecode = hex::encode(proxy_contract.code());

    println!(
        " - Proxy Contract Predicted ContractId: {}",
        &proxy_contract_id
    );

    println!(
        " - Proxy Contract Predicted Bytecode: {}",
        &proxy_contract_bytecode
    );
}
