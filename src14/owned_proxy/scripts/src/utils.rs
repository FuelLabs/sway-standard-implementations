use fuels::{
    accounts::{provider::Provider, wallet::WalletUnlocked},
    crypto::SecretKey,
    macros::abigen,
    programs::responses::CallResponse,
    types::ContractId,
};
use std::str::FromStr;

abigen!(Contract(
    name = "ProxyContract",
    abi = "../contract/out/release/src14_owned_proxy-abi.json"
));

pub mod proxy_abi_calls {

    use super::{CallResponse, ContractId, ProxyContract, State, WalletUnlocked};

    pub async fn set_proxy_target(
        contract: &ProxyContract<WalletUnlocked>,
        new_target: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .set_proxy_target(new_target)
            .call()
            .await
            .unwrap()
    }

    pub async fn proxy_target(
        contract: &ProxyContract<WalletUnlocked>,
    ) -> CallResponse<Option<ContractId>> {
        contract.methods().proxy_target().call().await.unwrap()
    }

    pub async fn proxy_owner(contract: &ProxyContract<WalletUnlocked>) -> CallResponse<State> {
        contract.methods().proxy_owner().call().await.unwrap()
    }

    pub async fn initialize_proxy(contract: &ProxyContract<WalletUnlocked>) -> CallResponse<()> {
        contract.methods().initialize_proxy().call().await.unwrap()
    }

    pub async fn set_proxy_owner(
        contract: &ProxyContract<WalletUnlocked>,
        new_proxy_owner: State,
    ) -> CallResponse<()> {
        contract
            .methods()
            .set_proxy_owner(new_proxy_owner)
            .call()
            .await
            .unwrap()
    }
}

pub async fn setup_signing_wallet(provider_url: &str, signing_key: &str) -> WalletUnlocked {
    let provider = Provider::connect(provider_url).await.unwrap();
    let secret = SecretKey::from_str(signing_key).unwrap();
    WalletUnlocked::new_from_private_key(secret, Some(provider))
}

pub fn get_proxy_instance(
    proxy_contract_id: &str,
    signing_wallet: WalletUnlocked,
) -> ProxyContract<WalletUnlocked> {
    let proxy_contract_id =
        ContractId::from_str(proxy_contract_id).expect("Proxy Contract Id could not be parsed");
    ProxyContract::new(proxy_contract_id, signing_wallet)
}
