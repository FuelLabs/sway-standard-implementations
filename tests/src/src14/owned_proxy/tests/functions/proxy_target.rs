mod success {

    use crate::src14::owned_proxy::tests::utils::{
        proxy_abi_calls::{initialize_proxy, proxy_target},
        test_helpers::setup,
    };
    use fuels::types::errors::Result;
    #[tokio::test]
    async fn returns_initialized_target() -> Result<()> {
        let (_deployer, owner1, _owner2, initial_target_id) = setup().await?;

        assert_eq!(proxy_target(&owner1.proxy_contract).await?.value, None);

        initialize_proxy(&owner1.proxy_contract).await?;

        assert_eq!(
            proxy_target(&owner1.proxy_contract).await?.value,
            Some(initial_target_id)
        );
        Ok(())
    }
}
