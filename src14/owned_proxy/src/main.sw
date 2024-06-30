contract;

mod interface;

use interface::OwnedProxy;
use ::sway_libs::{
    ownership::errors::InitializationError,
    upgradeability::{
        _proxy_owner,
        _proxy_target,
        _set_proxy_owner,
        _set_proxy_target,
        only_proxy_owner,
    },
};
use standards::{src14::{SRC14, SRC14Extension}, src5::State};
use std::execution::run_external;

configurable {
    INITIAL_TARGET: Option<ContractId> = None,
    INITIAL_OWNER: State = State::Uninitialized,
}

#[namespace(SRC14)]
storage {
    // target is at sha256("storage_SRC14_0")
    target: Option<ContractId> = None,
    // proxy_owner is at sha256("storage_SRC14_1")
    proxy_owner: State = State::Uninitialized,
}

impl SRC14 for Contract {
    #[storage(read, write)]
    fn set_proxy_target(new_target: ContractId) {
        only_proxy_owner(storage.proxy_owner);
        _set_proxy_target(new_target);
    }

    #[storage(read)]
    fn proxy_target() -> Option<ContractId> {
        _proxy_target()
    }
}

impl SRC14Extension for Contract {
    #[storage(read)]
    fn proxy_owner() -> State {
        _proxy_owner(storage.proxy_owner)
    }
}

impl OwnedProxy for Contract {
    #[storage(write)]
    fn initialize_proxy() {
        require(
            _proxy_owner(storage.proxy_owner) == State::Uninitialized,
            InitializationError::CannotReinitialized,
        );

        storage.target.write(INITIAL_TARGET);
        storage.proxy_owner.write(INITIAL_OWNER);
    }

    #[storage(write)]
    fn set_proxy_owner(new_proxy_owner: State) {
        _set_proxy_owner(new_proxy_owner, storage.proxy_owner);
    }
}

#[fallback]
#[storage(read)]
fn fallback() {
    run_external(_proxy_target().expect("FallbackError::TargetNotSet"))
}
