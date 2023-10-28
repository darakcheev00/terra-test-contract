
# Warp Test Contract - Hydrating Messages

This smart contract is designed to hydrate string messages by replacing variables with their respective values. The variable mapping (name: value) is provided by the user. All sub-messages of the main message, (base64-encoded and plaintext), are also hydrated.

## Smart Contract Messages

The following messages can be used with the contract:

### InstantiateMsg

Used to instantiate the smart contract.

Example:

```rust
    use contract::InstantiateMsg;
    let instantiate_msg = InstantiateMsg {};
```


### ExecuteMsg

#### HydrateMsg
This variant is used for message hydration, taking two fields:

- **msg**: A string that represents the message to be hydrated.
- **vars**: A string that represents the variables required for hydration.

Example:

```rust
    use contract::ExecuteMsg;
    let hydrate_msg = ExecuteMsg::HydrateMessage {
        msg: "Sample message",
        vars: "Sample variables",
    };
```
## Smart Contract Response

The response of the smart contract contains a CosmosMsg that was hydrated using the given variable name-value mapping.


## Running Tests

#### Tests

```test_instantiate```
- Tests instantiating the smart contract

```test_hydrate_msg_wasm_execute```
- Tests hydrating a Wasm Execute message

```test_hydrate_msg_ibc_send_packet```
- Tests hydrating an IBC SendPacket message

To run tests, run the following command

```bash
  cargo test
```

