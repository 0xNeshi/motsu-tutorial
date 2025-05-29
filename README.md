# Motsu Tutorial

Repo containing all the code examples from the "Testing Arbitrum Stylus Smart Contracts with Motsu" article.

Due to the limitation of Stylus SDK that the `#[entrypoint]` attribute can be defined on a single contract in a project, all contracts use feature gates to enable this attribute. To perform any operations with `cargo-stylus` on a specific contract, enable the appropriate feature.

```bash
# this will treat `erc20::MyToken` as the entrypoint
cargo stylus check --features erc20

# this will treat `vault::Vault` as the entrypoint
cargo stylus check --features vault

# this will treat `vm_env::MyChainAwareContract` as the entrypoint
cargo stylus check --features vm_env

# this will treat `contract_to_contract::Proxy` as the entrypoint
cargo stylus check --features contract_to_contract
```

## License

This project is fully open source, including an Apache-2.0 or MIT license at your choosing under your own copyright.
