use openzeppelin_stylus::token::erc20::{self, Erc20, IErc20};
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::*,
};

#[cfg_attr(feature = "erc20", entrypoint)]
#[storage]
struct MyToken {
    erc20: Erc20,
}

#[public]
#[implements(IErc20<Error = erc20::Error>)]
impl MyToken {
    fn mint(&mut self, account: Address, value: U256) -> Result<(), erc20::Error> {
        self.erc20._mint(account, value)
    }
}

#[public]
impl IErc20 for MyToken {
    type Error = erc20::Error;

    fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    fn balance_of(&self, account: Address) -> U256 {
        self.erc20.balance_of(account)
    }

    fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Self::Error> {
        self.erc20.transfer(to, value)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowance(owner, spender)
    }

    fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Self::Error> {
        self.erc20.approve(spender, value)
    }

    fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Self::Error> {
        self.erc20.transfer_from(from, to, value)
    }
}

#[cfg(test)]
mod tests {
    use motsu::prelude::*;
    use stylus_sdk::alloy_primitives::{Address, U256};

    use super::MyToken;
    use openzeppelin_stylus::token::erc20::{self, IErc20, Transfer};

    #[cfg(not(feature = "erc20"))]
    unsafe impl stylus_sdk::testing::TopLevelStorage for MyToken {}

    #[motsu::test]
    fn test_token_transfers(token: Contract<MyToken>, alice: Address, bob: Address) {
        let amount = U256::from(100);

        // Initialize token and mint to alice
        token.sender(alice).mint(alice, amount).motsu_unwrap();

        // Check initial balances
        assert_eq!(token.sender(alice).erc20.balance_of(alice), amount);
        assert_eq!(token.sender(alice).erc20.balance_of(bob), U256::ZERO);

        // Transfer tokens
        let transfer_amount = U256::from(30);
        token
            .sender(alice)
            .erc20
            .transfer(bob, transfer_amount)
            .motsu_unwrap();

        // Check updated balances
        assert_eq!(
            token.sender(alice).erc20.balance_of(alice),
            amount - transfer_amount
        );
        assert_eq!(token.sender(alice).erc20.balance_of(bob), transfer_amount);

        // Verify event was emitted
        token.assert_emitted(&Transfer {
            from: alice,
            to: bob,
            value: transfer_amount,
        });

        // Test failure case - transfer more than balance
        let too_much = amount * U256::from(2);
        let err = token
            .sender(alice)
            .erc20
            .transfer(bob, too_much)
            .motsu_unwrap_err();
        assert!(matches!(err, erc20::Error::InsufficientBalance(_)));
    }
}
