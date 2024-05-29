# hello-near

## Build and Test
Building and testing the contract is as simple as running two commands.

```bash
cargo build
```

```bash
cargo test
```

## Deploy the Contract
Having our account created, we can now deploy the contract into it:

```bash
near contract deploy yashtotla.testnet use-file ./target/wasm32-unknown-unknown/release/hello_near.wasm without-init-call network-config testnet sign-with-keychain send
```

## Interacting with the Contract
To interact with your deployed smart contract, you can call its methods using the near-cli-rs tools.

### Get Greeting
The get_greeting method is a view method, meaning it only reads from the contract's state, and can thus be called for free.

```bash
near contract call-function as-read-only yashtotla.testnet get_greeting json-args {} network-config testnet now
```

### Set Greeting
The set_greeting method is a change method, meaning it modifies the contract's state, and thus requires a user to sign a transaction in order to be executed.

```bash
near contract call-function as-transaction yashtotla.testnet set_greeting json-args '{"greeting": "Hola"}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as yashtotla.testnet network-config testnet sign-with-keychain send
```
