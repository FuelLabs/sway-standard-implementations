use crate::src14::owned_proxy::tests::utils::{
    proxy_abi_calls::{initialize_proxy, proxy_owner, proxy_target},
    test_helpers::setup,
    State,
};

mod success {

    use super::*;

    #[tokio::test]
    async fn only_owner_may_call() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        assert_eq!(proxy_target(&owner1.proxy_contract).await.value, None);
        assert_eq!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Uninitialized
        );

        initialize_proxy(&owner1.proxy_contract).await;

        assert_ne!(proxy_target(&owner1.proxy_contract).await.value, None);
        assert_ne!(
            proxy_owner(&owner1.proxy_contract).await.value,
            State::Uninitialized
        );
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialized")]
    async fn when_not_owner() {
        let (_deployer, owner1, _owner2, _initial_target_id) = setup().await;

        initialize_proxy(&owner1.proxy_contract).await;

        initialize_proxy(&owner1.proxy_contract).await;
    }
}
