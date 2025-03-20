use openzeppelin_stylus::token::erc20::{self, Erc20};
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::*,
};

#[entrypoint]
#[storage]
struct MyToken {
    #[borrow]
    erc20: Erc20,
}

#[public]
#[inherit(Erc20)]
impl MyToken {
    fn mint(&mut self, account: Address, value: U256) -> Result<(), erc20::Error> {
        self.erc20._mint(account, value)
    }
}

#[cfg(test)]
mod tests {
    use motsu::prelude::*;
    use stylus_sdk::alloy_primitives::{Address, U256};

    use super::MyToken;
    use openzeppelin_stylus::token::erc20::{self, IErc20, Transfer};

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
        // token.assert_emitted(&Transfer {
        //     from: alice,
        //     to: bob,
        //     value: transfer_amount,
        // });

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
