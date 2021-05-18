## Rust Setup

Follow [this link](https://www.rust-lang.org/tools/install) to setup Rust in your system.

## Build

Run this command inside the provenance-usecase/substrate folder to build the binary file.

```sh
cargo build --release
```

## Run

Use this command to start the substrate node.

```sh
./target/release/provenance-substrate --dev --tmp --alice --ws-external --rpc-external
```

## Type definitions for Polkadot JS Portal

To connect the local substrate node with the [Polkadot JS Portal](https://portal.chain.centrifuge.io/#/explorer), click on the top-left corner of the portal and select DEVELOPMENT -> Local Node.

Use these type definitions at Settings -> Developer: 

```json

{
  "MemberType": {
    "_enum": [
      "Manufacturer",
      "Carrier",
      "Retailer",
      "Customer"
    ]
  },
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "BottleId": "Vec<u8>",
  "Bottle<AccountId, Moment>": {
    "id": "BottleId",
    "manufacturer": "AccountId",
    "registered": "Moment"
  },
  "ShipmentId": "Vec<u8>",
  "ShipmentStatus": {
    "_enum": [
      "Pending",
      "InTransit",
      "Delivered"
    ]
  },
  "ShipmentOperation": {
    "_enum": [
      "Pickup",
      "Scan",
      "Deliver"
    ]
  },
  "Shipment<AccountId, Moment>": {
    "id": "ShipmentId",
    "manufacturer": "AccountId",
    "carrier": "AccountId",
    "retailer": "AccountId",
    "bottles": "Vec<BottleId>",
    "status": "ShipmentStatus",
    "timestamp": "Moment",
    "delivered": "Option<Moment>"
  },
  "chainbridge::ChainId": "u8",
  "ChainId": "u8",
  "ResourceId": "[u8; 32]",
  "DepositNonce": "u64",
  "ProposalVotes": {
    "votes_for": "Vec<AccountId>",
    "votes_against": "Vec<AccountId>",
    "status": "enum"
  },
  "Erc721Token": {
    "id": "TokenId",
    "metadata": "Vec<u8>"
  },
  "TokenId": "U256"
}

```
