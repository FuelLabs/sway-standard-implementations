mod success {

    use crate::src14::owned_proxy::tests::utils::{
        proxy_abi_calls::{initialize_proxy, proxy_owner, set_proxy_owner},
        test_helpers::setup,
        State,
    };

    #[tokio::test]
    async fn returns_initialized_owner() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Uninitialized
        );

        initialize_proxy(&owner1.proxy_contract).await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Initialized(owner1.wallet.address().into())
        );
    }

    #[tokio::test]
    async fn returns_owner_on_state_change() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        initialize_proxy(&owner1.proxy_contract).await;

        set_proxy_owner(&owner1.proxy_contract, State::Revoked).await;

        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Revoked
        );
    }
}
