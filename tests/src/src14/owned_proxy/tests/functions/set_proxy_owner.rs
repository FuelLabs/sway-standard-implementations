use crate::src14::owned_proxy::tests::utils::{
    proxy_abi_calls::{initialize_proxy, proxy_owner, set_proxy_owner},
    test_helpers::setup,
    State,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn sets_a_new_owner() {
        let (_deployer, owner1, owner2, _initial_target_id) = setup().await;

        initialize_proxy(&owner1.proxy_contract).await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Initialized(owner1.wallet.address().into())
        );

        set_proxy_owner(
            &owner1.proxy_contract,
            State::Initialized(owner2.wallet.address().into()),
        )
        .await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Initialized(owner2.wallet.address().into())
        );
    }

    #[tokio::test]
    async fn revokes_ownership() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        initialize_proxy(&owner1.proxy_contract).await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Initialized(owner1.wallet.address().into())
        );

        set_proxy_owner(&owner1.proxy_contract, State::Revoked).await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Revoked
        );
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_called_by_non_owner() {
        let (_deployer, _owner1, owner2, _initial_target_id) = setup().await;

        initialize_proxy(&owner2.proxy_contract).await;

        set_proxy_owner(
            &owner2.proxy_contract,
            State::Initialized(owner2.wallet.address().into()),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotUninitialize")]
    async fn when_setting_the_new_state_to_uninitialized() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        initialize_proxy(&owner1.proxy_contract).await;

        set_proxy_owner(&owner1.proxy_contract, State::Uninitialized).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_initialized() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        set_proxy_owner(&owner1.proxy_contract, State::Revoked).await;
    }
}
