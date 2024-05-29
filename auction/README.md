# auction

## Deploy the Contract
Having our account created, we can now deploy the contract into it:

```bash
near contract deploy yashtotla.testnet use-file ./target/wasm32-unknown-unknown/release/auction.wasm without-init-call network-config testnet sign-with-keychain send
```

Transaction ID: ```F3htsTvDTipGTAZR2z8cxYFnxTQNfPq2LHg45tuseZap```

To see the transaction in the transaction explorer, please open this url in your browser:
https://explorer.testnet.near.org/transactions/F3htsTvDTipGTAZR2z8cxYFnxTQNfPq2LHg45tuseZap
