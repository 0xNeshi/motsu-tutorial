use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::Call,
    prelude::*,
    storage::{StorageAddress, StorageU256},
};

#[storage]
struct MyContract {
    value: StorageU256,
}

#[public]
impl MyContract {
    fn set_value(&mut self, new_value: U256) {
        self.value.set(new_value);
    }
}

// As MyContract is not annotated with #[entrypoint],
// we must implement TopLevelStorage manually
unsafe impl TopLevelStorage for MyContract {}

sol_interface! {
    interface IMyContract {
        #[allow(missing_docs)]
        function setValue(uint256 new_value) external;
    }
}

#[storage]
#[entrypoint]
struct Proxy {
    implementation: StorageAddress,
}

#[public]
impl Proxy {
    fn set_implementation(&mut self, protected: Address) {
        self.implementation.set(protected);
    }

    fn set_value(&mut self, new_value: U256) {
        let my_contract = IMyContract::new(self.implementation.get());
        let call = Call::new_in(self);
        my_contract
            .set_value(call, new_value)
            .expect("should pass the call");
    }
}

#[cfg(test)]
mod tests {
    use motsu::prelude::*;
    use stylus_sdk::alloy_primitives::{Address, U256};

    use super::{MyContract, Proxy};

    #[motsu::test]
    fn test_proxy_call(proxy: Contract<Proxy>, my_contract: Contract<MyContract>, alice: Address) {
        // set appropriate proxy implementation
        proxy
            .sender(alice)
            .set_implementation(my_contract.address());

        // assert value is zero
        let value = my_contract.sender(alice).value.get();
        assert_eq!(value, U256::ZERO);

        proxy.sender(alice).set_value(U256::from(100));

        // assert value is updated to 100
        let value = my_contract.sender(alice).value.get();
        assert_eq!(value, U256::from(100));
    }
}
