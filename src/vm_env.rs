use stylus_sdk::{block, prelude::*};

#[storage]
#[entrypoint]
struct MyChainAwareContract;

#[public]
impl MyChainAwareContract {
    fn get_chain_id(&self) -> u64 {
        block::chainid()
    }
}

#[cfg(test)]
mod tests {
    use motsu::prelude::*;
    use stylus_sdk::alloy_primitives::Address;

    use super::MyChainAwareContract;

    #[motsu::test]
    fn test_with_custom_chain_id(contract: Contract<MyChainAwareContract>, alice: Address) {
        // Default is Arbitrum One (42161)
        assert_eq!(contract.sender(alice).get_chain_id(), 42161);

        // Change to Sepolia testnet
        VMContext::current().set_chain_id(11155111);

        // Verify the contract now sees the new chain ID
        assert_eq!(contract.sender(alice).get_chain_id(), 11155111);
    }
}
