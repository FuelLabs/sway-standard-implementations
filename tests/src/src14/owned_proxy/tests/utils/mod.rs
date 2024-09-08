use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
    },
    programs::responses::CallResponse,
    types::bech32::Bech32ContractId,
};

use fuels::types::errors::Result;

// Load abi from json
abigen!(Contract(
    name = "ProxyContract",
    abi = "../src14/owned_proxy/contract/out/release/src14_owned_proxy-abi.json"
),Contract(
    name = "TargetContract",
    abi =
        "src/src14/owned_proxy/test_artifacts/target_contract/out/release/target_contract-abi.json"
));

pub struct Metadata {
    pub proxy_contract: ProxyContract<WalletUnlocked>,
    pub target_contract: TargetContract<WalletUnlocked>,
    pub wallet: WalletUnlocked,
}

pub mod proxy_abi_calls {

    use super::*;

    pub async fn set_proxy_target(
        contract: &ProxyContract<WalletUnlocked>,
        new_target: ContractId,
    ) -> Result<CallResponse<()>> {
        contract.methods().set_proxy_target(new_target).call().await
    }

    pub async fn proxy_target(
        contract: &ProxyContract<WalletUnlocked>,
    ) -> Result<CallResponse<Option<ContractId>>> {
        contract.methods().proxy_target().call().await
    }

    pub async fn proxy_owner(
        contract: &ProxyContract<WalletUnlocked>,
    ) -> Result<CallResponse<State>> {
        contract.methods().proxy_owner().call().await
    }

    pub async fn initialize_proxy(
        contract: &ProxyContract<WalletUnlocked>,
    ) -> Result<CallResponse<()>> {
        contract.methods().initialize_proxy().call().await
    }

    pub async fn set_proxy_owner(
        contract: &ProxyContract<WalletUnlocked>,
        new_proxy_owner: State,
    ) -> Result<CallResponse<()>> {
        contract
            .methods()
            .set_proxy_owner(new_proxy_owner)
            .call()
            .await
    }
}

pub mod target_abi_calls {

    use super::*;

    pub async fn sum(
        contract: &TargetContract<WalletUnlocked>,
        implementation_contract_id: Bech32ContractId,
        a: u64,
        b: u64,
    ) -> Result<CallResponse<u64>> {
        contract
            .methods()
            .sum(a, b)
            .with_contract_ids(&[implementation_contract_id])
            .call()
            .await
    }

    pub async fn read_amount(
        contract: &TargetContract<WalletUnlocked>,
        implementation_contract_id: Bech32ContractId,
    ) -> Result<CallResponse<u64>> {
        contract
            .methods()
            .read_amount()
            .with_contract_ids(&[implementation_contract_id])
            .call()
            .await
    }

    pub async fn increment_amount(
        contract: &TargetContract<WalletUnlocked>,
        implementation_contract_id: Bech32ContractId,
    ) -> Result<CallResponse<()>> {
        contract
            .methods()
            .increment_amount()
            .with_contract_ids(&[implementation_contract_id])
            .call()
            .await
    }
}

pub mod test_helpers {

    use super::*;
    use fuels::test_helpers::{ChainConfig, NodeConfig};

    pub const INITIAL_INCREMENT_AMOUNT: u64 = 1;

    pub async fn deploy_target_contract(
        deployer_wallet: &WalletUnlocked,
        increment_amount: u64,
    ) -> Result<ContractId> {
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(
                "src/src14/owned_proxy/test_artifacts/target_contract/out/release/target_contract-storage_slots.json",
            )?;

        let configurables =
            TargetContractConfigurables::default().with_INCREMENT_AMOUNT(increment_amount)?;

        let configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration)
            .with_configurables(configurables);

        Ok(Contract::load_from(
            "src/src14/owned_proxy/test_artifacts/target_contract/out/release/target_contract.bin",
            configuration,
        )?
        .deploy(deployer_wallet, TxPolicies::default())
        .await?
        .into())
    }

    pub async fn setup() -> Result<(Metadata, Metadata, Metadata, ContractId)> {
        let num_wallets = 3;
        let coins_per_wallet = 1;
        let coin_amount = 1_000_000;

        let mut wallets = launch_custom_provider_and_get_wallets(
            WalletsConfig::new(Some(num_wallets), Some(coins_per_wallet), Some(coin_amount)),
            Some(NodeConfig::default()),
            Some(ChainConfig::default()),
        )
        .await?;
        // Get the wallets from that provider
        let deploy_wallet = wallets.pop().unwrap();
        let owner1 = wallets.pop().unwrap();
        let owner2 = wallets.pop().unwrap();

        // deploy initial target contract
        let initial_target_id =
            deploy_target_contract(&deploy_wallet, INITIAL_INCREMENT_AMOUNT).await?;

        // deploy proxy that targets the initial target contract
        let storage_configuration = StorageConfiguration::default().add_slot_overrides_from_file(
            "../src14/owned_proxy/contract/out/release/src14_owned_proxy-storage_slots.json",
        )?;

        let configurables = ProxyContractConfigurables::default()
            .with_INITIAL_TARGET(Some(initial_target_id))?
            .with_INITIAL_OWNER(State::Initialized(owner1.address().into()))?;

        let configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration)
            .with_configurables(configurables);

        let proxy_id = Contract::load_from(
            "../src14/owned_proxy/contract/out/release/src14_owned_proxy.bin",
            configuration,
        )?
        .deploy(&deploy_wallet, TxPolicies::default())
        .await?;

        // Use the proxy_id for both contracts; ensuring that all calls go to the proxy.
        let deployer = Metadata {
            proxy_contract: ProxyContract::new(proxy_id.clone(), deploy_wallet.clone()),
            target_contract: TargetContract::new(proxy_id.clone(), deploy_wallet.clone()),
            wallet: deploy_wallet.clone(),
        };

        let owner1 = Metadata {
            proxy_contract: ProxyContract::new(proxy_id.clone(), owner1.clone()),
            target_contract: TargetContract::new(proxy_id.clone(), owner1.clone()),
            wallet: owner1.clone(),
        };

        let owner2 = Metadata {
            proxy_contract: ProxyContract::new(proxy_id.clone(), owner2.clone()),
            target_contract: TargetContract::new(proxy_id, owner2.clone()),
            wallet: owner2.clone(),
        };

        Ok((deployer, owner1, owner2, initial_target_id))
    }
}
