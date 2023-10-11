# Button Distributor

This is a smart contract to facilitate the safe distribution of Button ($BTN) on Aleph Zero. It is the final step of the Button ($BUTT) migration from Secret Network. All minted $BTN will be initially sent to this smart contract. With the amount and address details collected from the Secret Network migration contract, we will increase the account allowances accordingly.

### Features

1. Initialise with admin and $BTN smart contract details.
2. Allow admin to increase_allowance for an account and store the details. The key for details is a String as it will be an encryption of the corresponding Secret Network migration order id.
3. Allow admin to decrease_allowance for an order and remove the details. This is in case the user has entered the wrong address.
4. Allow user to collect their $BTN.
5. Allow retrieving order details.

## Getting Started
### Prerequisites

* [Cargo](https://doc.rust-lang.org/cargo/)
* [Rust](https://www.rust-lang.org/)
* [ink!](https://use.ink/)
* [OpenBrush](https://openbrush.io/)
* [Cargo Contract v2.0.1](https://github.com/paritytech/cargo-contract)
```zsh
cargo install --force --locked cargo-contract --version 2.0.1
```

### Checking code

```zsh
cargo checkmate
cargo sort
```

## Deployment

1. Build contract:
```sh
cargo contract build --release
```
2. If setting up locally, start a local development chain.
```sh
substrate-contracts-node --dev
```
3. Upload, initialise and interact with contract at [Contracts UI](https://contracts-ui.substrate.io/).

## References

- https://use.ink/basics/cross-contract-calling#builders
- https://github.com/Brushfam/openbrush-contracts/blob/a255c212debdace8b82f84cb104e84b716ccd6ac/contracts/src/token/psp22_pallet/psp22_pallet.rs#L125
- https://github.com/RottenKiwi/Panorama-Swap-INK-SC/blob/a811b10eb9e108192686e6d13ffcd6ad0fddd533/staking_contact/lib.rs#L348
- https://docs.rs/ink_env/latest/ink_env/call/struct.CallBuilder.html#method.try_invoke-2

