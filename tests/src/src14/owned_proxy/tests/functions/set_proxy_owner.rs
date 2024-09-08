use crate::src14::owned_proxy::tests::utils::{
    proxy_abi_calls::{initialize_proxy, proxy_owner, set_proxy_owner},
    test_helpers::setup,
    State,
};

use fuels::types::errors::Result;

mod success {

    use super::*;

    #[tokio::test]
    async fn sets_a_new_owner() -> Result<()> {
        let (_deployer, owner1, owner2, _initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await?.value,
            State::Initialized(owner1.wallet.address().into())
        );

        set_proxy_owner(
            &owner1.proxy_contract,
            State::Initialized(owner2.wallet.address().into()),
        )
        .await?;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await?.value,
            State::Initialized(owner2.wallet.address().into())
        );

        Ok(())
    }

    #[tokio::test]
    async fn revokes_ownership() -> Result<()> {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await?.value,
            State::Initialized(owner1.wallet.address().into())
        );

        set_proxy_owner(&owner1.proxy_contract, State::Revoked).await?;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await?.value,
            State::Revoked
        );
        Ok(())
    }
}

mod reverts {
    use super::*;
    use fuels::prelude::Error;
    use fuels::types::errors::transaction::Reason;

    #[tokio::test]
    async fn when_called_by_non_owner() -> Result<()> {
        let (_deployer, _owner1, owner2, _initial_target_id) = setup().await?;

        initialize_proxy(&owner2.proxy_contract).await?;

        let err = set_proxy_owner(
            &owner2.proxy_contract,
            State::Initialized(owner2.wallet.address().into()),
        )
        .await;

        assert!(matches!(
            err.unwrap_err(),
            Error::Transaction(
                Reason::Reverted { reason, .. }
            ) if reason == "NotOwner"
        ));

        Ok(())
    }

    #[tokio::test]
    async fn when_setting_the_new_state_to_uninitialized() -> Result<()> {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        let err = set_proxy_owner(&owner1.proxy_contract, State::Uninitialized).await;

        assert!(matches!(
            err.unwrap_err(),
            Error::Transaction(
                Reason::Reverted { reason, .. }
            ) if reason == "CannotUninitialize"
        ));

        Ok(())
    }

    #[tokio::test]
    async fn when_not_initialized() -> Result<()> {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await?;

        let err = set_proxy_owner(&owner1.proxy_contract, State::Revoked).await;

        assert!(matches!(
            err.unwrap_err(),
            Error::Transaction(
                Reason::Reverted { reason, .. }
            ) if reason == "NotOwner"
        ));

        Ok(())
    }
}
