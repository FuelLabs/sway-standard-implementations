use crate::src14::owned_proxy::tests::utils::{
    proxy_abi_calls::{initialize_proxy, set_proxy_target},
    target_abi_calls::{increment_amount, read_amount, sum},
    test_helpers::{deploy_target_contract, setup, INITIAL_INCREMENT_AMOUNT},
};

use fuels::types::errors::Result;

mod success {

    use super::*;

    const SECOND_INCREMENT_AMOUNT: u64 = 20 * INITIAL_INCREMENT_AMOUNT;

    #[tokio::test]
    async fn calls_pure_method_with_initial_target() -> Result<()> {
        let (_deployer, owner1, _owner2, initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        // Use target contract method at proxy contract ID
        assert_eq!(
            sum(&owner1.target_contract, initial_target_id.into(), 1, 2)
                .await?
                .value,
            3
        );

        Ok(())
    }

    #[tokio::test]
    async fn calls_read_and_write_methods_with_initial_target() -> Result<()> {
        let (_deployer, owner1, _owner2, initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        // Use target contract methods at proxy contract ID
        increment_amount(&owner1.target_contract, initial_target_id.into()).await?;

        assert_eq!(
            read_amount(&owner1.target_contract, initial_target_id.into())
                .await?
                .value,
            INITIAL_INCREMENT_AMOUNT
        );

        increment_amount(&owner1.target_contract, initial_target_id.into()).await?;

        assert_eq!(
            read_amount(&owner1.target_contract, initial_target_id.into())
                .await?
                .value,
            2 * INITIAL_INCREMENT_AMOUNT
        );

        Ok(())
    }

    #[tokio::test]
    async fn calls_pure_method_with_second_target() -> Result<()> {
        let (deployer, owner1, _owner2, initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        let second_target_id =
            deploy_target_contract(&deployer.wallet, SECOND_INCREMENT_AMOUNT).await?;

        assert_ne!(initial_target_id, second_target_id);

        set_proxy_target(&owner1.proxy_contract, second_target_id).await?;

        // Use target contract method at proxy contract ID
        assert_eq!(
            sum(&owner1.target_contract, second_target_id.into(), 1, 2)
                .await?
                .value,
            3
        );

        Ok(())
    }

    #[tokio::test]
    async fn calls_read_and_write_methods_with_second_target() -> Result<()> {
        let (deployer, owner1, _owner2, initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        let second_target_id =
            deploy_target_contract(&deployer.wallet, SECOND_INCREMENT_AMOUNT).await?;

        assert_ne!(initial_target_id, second_target_id);

        set_proxy_target(&owner1.proxy_contract, second_target_id).await?;

        // Use target contract methods at proxy contract ID
        increment_amount(&owner1.target_contract, second_target_id.into()).await?;

        assert_eq!(
            read_amount(&owner1.target_contract, second_target_id.into())
                .await?
                .value,
            SECOND_INCREMENT_AMOUNT
        );

        increment_amount(&owner1.target_contract, second_target_id.into()).await?;

        assert_eq!(
            read_amount(&owner1.target_contract, second_target_id.into())
                .await?
                .value,
            2 * SECOND_INCREMENT_AMOUNT
        );
        Ok(())
    }
}

mod reverts {
    use super::*;
    use fuels::prelude::Error;
    use fuels::types::errors::transaction::Reason;

    #[tokio::test]
    async fn when_target_is_not_set() -> Result<()> {
        let (_deployer, owner1, _owner2, initial_target_id) = setup().await?;

        let err = read_amount(&owner1.target_contract, initial_target_id.into()).await;

        assert!(matches!(
            err.unwrap_err(),
            Error::Transaction(Reason::Reverted { reason, .. } ) if reason == "Revert(0)"
        ));

        Ok(())
    }
}
