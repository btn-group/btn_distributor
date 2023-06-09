<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/btn-group">
    <img src="images/logo.png" alt="Logo" height="80">
  </a>

  <h3 align="center">Button ($BTN) Distributor</h3>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#setting-up-locally">Setting up locally</a></li>
      </ul>
    </li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

This is a smart contract to facilitate the safe distribution of Button ($BTN) on Aleph Zero. It is the final step of the Button ($BUTT) migration from Secret Network. All minted $BTN will be initially sent to this smart contract. With the amount and address details collected from the Secret Network migration contract, we will increase the account allowances accordingly.

### Features

1. Initialise with admin and $BTN smart contract details.
2. Allow admin to increase_allowance for an account and store the details. The key for details is a String as it will be an encryption of the corresponding Secret Network migration order id.
3. Allow admin to decrease_allowance for an order and remove the details. This is in case the user has entered the wrong address.
4. Allow user to collect their $BTN.
5. Allow retrieving order details.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

* [Cargo](https://doc.rust-lang.org/cargo/)
* [Rust](https://www.rust-lang.org/)
* [ink!](https://use.ink/)
* [OpenBrush](https://openbrush.io/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple example steps.

* Open brush required that I use rust nightly but building the contract required stable.

### Prerequisites

* A pre-requisite for compiling smart contracts is to have a stable Rust version and Cargo installed. Here's an [installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html).
* The first tool we will be installing is [cargo-contract](https://github.com/paritytech/cargo-contract), a CLI tool for helping setting up and managing WebAssembly smart contracts written with ink!.

### Building contract

By default, cargo-contract builds the contract in debug mode. This means that the contract will e.g. print statements like

```sh
ink::env::debug_println!("magic number: {}", value);
```
to the node's console if debugging was enabled on the node ([instructions here](https://use.ink/faq#how-do-i-print-something-to-the-console-from-the-runtime)). To support functionality like this the debug build of a contract includes some heavy-weight logic.

For contracts that are supposed to run in production you should always build the contract with --release:
```sh
cargo +stable contract build --release
```
This will ensure that nothing unnecessary is compiled into the Wasm blob, making your contract faster and cheaper to deploy and execute.

### Setting up locally

The [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node) is a simple Substrate blockchain which is configured to include the Substrate module for smart contract functionality – the contracts pallet (see [How it Works](https://use.ink/how-it-works) for more). It's a comfortable option if you want to get a quickstart. Download the binary [here](https://github.com/paritytech/substrate-contracts-node/releases).

[After successfully installing substrate-contracts-node](https://use.ink/getting-started/setup#installing-the-substrate-smart-contracts-node), you can start a local development chain by running:

```sh
substrate-contracts-node
```

You can interact with your node using the [Contracts UI](https://contracts-ui.substrate.io/). Once you have the webpage open, click on the dropdown selector at the top left corner and choose "Local Node".

Note that blocks are only created when you execute a function in substrate-contracts-node, so trigger a another function first if a function depends on a time delay.

## References

- https://use.ink/basics/cross-contract-calling#builders
- https://github.com/Brushfam/openbrush-contracts/blob/a255c212debdace8b82f84cb104e84b716ccd6ac/contracts/src/token/psp22_pallet/psp22_pallet.rs#L125
- https://github.com/RottenKiwi/Panorama-Swap-INK-SC/blob/a811b10eb9e108192686e6d13ffcd6ad0fddd533/staking_contact/lib.rs#L348
- https://docs.rs/ink_env/latest/ink_env/call/struct.CallBuilder.html#method.try_invoke-2

