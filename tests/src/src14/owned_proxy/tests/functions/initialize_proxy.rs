use crate::src14::owned_proxy::tests::utils::{
    proxy_abi_calls::{initialize_proxy, proxy_owner, proxy_target},
    test_helpers::setup,
    State,
};

use fuels::types::errors::Result;

mod success {

    use super::*;

    #[tokio::test]
    async fn only_owner_may_call() -> Result<()> {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await?;

        assert_eq!(proxy_target(&owner1.proxy_contract).await?.value, None);
        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await?.value,
            State::Uninitialized
        );

        initialize_proxy(&owner1.proxy_contract).await?;

        assert_ne!(proxy_target(&owner1.proxy_contract).await?.value, None);
        assert_ne!(
            proxy_owner(&owner1.proxy_contract).await?.value,
            State::Uninitialized
        );

        Ok(())
    }
}

mod reverts {
    use super::*;
    use fuels::prelude::Error;
    use fuels::types::errors::transaction::Reason;

    #[tokio::test]
    async fn when_not_owner() -> Result<()> {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await?;

        initialize_proxy(&owner1.proxy_contract).await?;

        let err = initialize_proxy(&owner1.proxy_contract).await;

        assert!(matches!(
            err.unwrap_err(),
            Error::Transaction(Reason::Reverted { reason, .. } ) if reason == "CannotReinitialized"
        ));
        Ok(())
    }
}
