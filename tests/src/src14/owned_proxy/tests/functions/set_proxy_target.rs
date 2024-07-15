use crate::src14::owned_proxy::tests::utils::{
    proxy_abi_calls::{initialize_proxy, proxy_target, set_proxy_target},
    test_helpers::{deploy_target_contract, setup, INITIAL_INCREMENT_AMOUNT},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn sets_a_new_target() {
        let (deployer, owner1, _owner2, initial_target_id) = setup().await;

        initialize_proxy(&owner1.proxy_contract).await;

        assert_eq!(
            proxy_target(&owner1.proxy_contract).await.value,
            Some(initial_target_id)
        );

        let second_target_id =
            deploy_target_contract(&deployer.wallet, 100 * INITIAL_INCREMENT_AMOUNT).await;

        assert_ne!(initial_target_id, second_target_id);

        set_proxy_target(&owner1.proxy_contract, second_target_id).await;

        assert_eq!(
            proxy_target(&owner1.proxy_contract).await.value,
            Some(second_target_id)
        );
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_called_by_non_owner() {
        let (_deployer, _owner1, owner2, initial_target_id) = setup().await;

        initialize_proxy(&owner2.proxy_contract).await;

        set_proxy_target(&owner2.proxy_contract, initial_target_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn when_not_initialized() {
        let (_deployer, owner1, _owner2, initial_target_id) = setup().await;

        set_proxy_target(&owner1.proxy_contract, initial_target_id).await;
    }
}
