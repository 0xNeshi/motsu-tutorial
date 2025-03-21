use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};
use stylus_sdk::{alloy_sol_types::sol, evm, msg};

sol! {
    #[derive(Debug)]
    event Deposited(address account, uint256 value);
}

sol! {
    #[derive(Debug)]
    error BalanceUnderflow(uint256 balance, uint256 needed);
}

#[derive(SolidityError, Debug)]
enum Error {
    BalanceUnderflow(BalanceUnderflow),
}

#[entrypoint]
#[storage]
struct Vault {
    balance: StorageU256,
}

#[public]
impl Vault {
    fn decrease_balance(&mut self, value: U256) -> Result<(), Error> {
        let previous_balance = self.balance.get();
        if previous_balance < value {
            return Err(BalanceUnderflow {
                balance: previous_balance,
                needed: value,
            }
            .into());
        }
        self.balance.set(previous_balance - value);
        Ok(())
    }

    #[payable]
    fn deposit(&mut self) {
        evm::log(Deposited {
            account: msg::sender(),
            value: msg::value(),
        });
    }

    fn get_balance(&self) -> U256 {
        self.balance.get()
    }

    fn increase_balance(&mut self, value: U256) {
        let previous_balance = self.balance.get();
        self.balance.set(previous_balance + value)
    }
}

#[cfg(test)]
mod tests {
    use motsu::prelude::*;
    use stylus_sdk::{
        alloy_primitives::{Address, U256},
        prelude::*,
    };

    use super::{BalanceUnderflow, Deposited, Error, Vault};

    #[motsu::test]
    fn reads_balance(contract: Contract<Vault>, alice: Address) {
        // Access storage
        let balance = contract.sender(alice).get_balance();
        assert_eq!(U256::ZERO, balance);
    }

    use alloy_signer::SignerSync;

    #[motsu::test]
    fn test_with_account(alice: Account) {
        // Use alice.address() to get the Address
        // Use alice.signer() to access signing capabilities
        let msg = "message".as_bytes();
        let signer = alice.signer();
        assert!(signer.sign_message_sync(msg).is_ok());
    }

    use alloy_primitives::address;

    const ALICE: Address = address!("0x328809bc894f92807417d2dad6b7c998c1afdac6");
    const BOB: Address = address!("0x38e47a7b719dce63662aeaf43440326f551b8a7e");
    const CONTRACT: Address = address!("0x7f6dd79f0020bee2024a097aaa5d32ab7ca31126");

    // These will always resolve to the same addresses across test runs
    #[motsu::test]
    fn test_with_injected_accounts(alice: Account, bob: Address, contract: Contract<Vault>) {
        assert_eq!(ALICE, alice.address());
        assert_eq!(BOB, bob);
        assert_eq!(CONTRACT, contract.address());
    }

    #[test]
    fn test_with_named_accounts() {
        let alice = Account::from_tag("alice");
        let bob = Address::from_tag("bob");
        let contract = Contract::<Vault>::from_tag("contract");

        assert_eq!(ALICE, alice.address());
        assert_eq!(BOB, bob);
        assert_eq!(CONTRACT, contract.address());
    }

    #[motsu::test]
    fn test_increase_balance(contract: Contract<Vault>, alice: Address) {
        let value = U256::from(100);

        contract.sender(alice).increase_balance(value);

        let new_balance = contract.sender(alice).get_balance();
        assert_eq!(new_balance, value);
    }

    #[motsu::test]
    fn test_payable_function(contract: Contract<Vault>, alice: Address) {
        // Fund alice first (she starts with zero balance)
        alice.fund(U256::from(10));

        // Call payable function with 1 wei
        contract.sender_and_value(alice, U256::from(1)).deposit();

        // Verify balances changed correctly
        assert_eq!(alice.balance(), U256::from(9));
        assert_eq!(contract.balance(), U256::from(1));
    }

    #[motsu::test]
    fn test_events(contract: Contract<Vault>, alice: Address) {
        // Fund alice first (she starts with zero balance)
        alice.fund(U256::from(10));

        // Perform an action that should emit an event
        contract.sender_and_value(alice, U256::from(1)).deposit();

        // Check that the event was emitted with correct parameters
        contract.assert_emitted(&Deposited {
            account: alice,
            value: U256::from(1),
        });

        // You can also just check without asserting
        let was_emitted = contract.emitted(&Deposited {
            account: alice,
            value: U256::from(1),
        });
        assert!(was_emitted);
    }

    #[motsu::test]
    #[should_panic = "event was not emitted, matching events: [Deposited { account: alice, value: 1 }]"]
    fn test_event_mismatch(contract: Contract<Vault>, alice: Address) {
        alice.fund(U256::from(10));

        contract.sender_and_value(alice, U256::from(1)).deposit();

        // Accidentally assert an event with wrong parameters was emitted
        contract.assert_emitted(&Deposited {
            account: alice,
            value: U256::from(100),
        });
    }

    #[motsu::test]
    fn test_revert(contract: Contract<Vault>, alice: Address) {
        // Attempt an operation that should revert
        let err = contract
            .sender(alice)
            .decrease_balance(U256::from(1000))
            .motsu_unwrap_err();

        // Check the error type
        assert!(matches!(
            err,
            Error::BalanceUnderflow(BalanceUnderflow {
                balance,
                needed,
            }) if balance == U256::ZERO && needed == U256::from(1000)
        ));
    }

    // Addresses for contract and alice are derived from tags with the same
    // values, so Motsu is able to substitute them in the panic message.
    #[motsu::test]
    #[should_panic = "account alice should fail to call contract"]
    fn test_panic_with_tags(contract: Contract<Vault>, alice: Address) {
        contract.sender(alice).increase_balance(U256::from(100));

        // Attempt an operation that should succeed
        _ = contract
            .sender(alice)
            .decrease_balance(U256::from(100))
            .motsu_unwrap_err();
    }

    // Since in this case tags were not used to instantiate addresses,
    // address values are used as-is in the panic message.
    #[test]
    #[should_panic = "account 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef should fail to call 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"]
    fn test_panic_no_tags() {
        let alice = address!("DeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF");
        let contract =
            Contract::<Vault>::new_at(address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"));

        contract.sender(alice).increase_balance(U256::from(100));

        // Attempt an operation that should succeed
        _ = contract
            .sender(alice)
            .decrease_balance(U256::from(100))
            .motsu_unwrap_err();
    }
}
