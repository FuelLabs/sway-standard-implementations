contract;

configurable {
    INCREMENT_AMOUNT: u64 = 1,
}

abi TargetContract {
    fn sum(a: u64, b: u64) -> u64;

    #[storage(read)]
    fn read_amount() -> u64;

    #[storage(write)]
    fn increment_amount();
}

storage {
    amount: u64 = 0,
}

impl TargetContract for Contract {
    fn sum(a: u64, b: u64) -> u64 {
        a + b
    }

    #[storage(read)]
    fn read_amount() -> u64 {
        storage.amount.read()
    }

    #[storage(write)]
    fn increment_amount() {
        storage.amount.write(storage.amount.try_read().unwrap_or(0) + INCREMENT_AMOUNT)
    }
}
