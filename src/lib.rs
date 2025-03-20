#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

mod contract_to_contract;
mod erc20;
mod vault;
mod vm_env;
