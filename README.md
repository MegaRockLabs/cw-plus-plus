# CW++
Experimental CosmWasm based contracts, packages and protocols 



## Standards



| Name                                      | Description                                                           |  Examples      |
| ----------------------------------------- | --------------------------------------------------------------------- | -------------- |
| [`cw-23`](./packages/cw23/)               | Signature verification for smart contracts (inspired by ERC-1271)     | [link](#cw-23) |


## Contracts

Example contracts showcasing usage of the proposed protocols and standards

### cw-23
| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw-23-last-signature`](./contracts/cw23-last-signature/)       | Contract owner stores a exprirable signaturen and verifications happens against it |
| [`cw-23-pubkey`](./contracts/cw23-pubkey/)                       | Using secp256k1 public key provided by contract creator and verifying using ecdsa  |
| [`cw-23-sn-ks`](./contracts/cw23-sn-ks/)                         | SecretWasm based contract using a secp256k1 private key for signature generation and verification |


<br/>

---


### Purpose
This is playground repository for MegaRock to test out new ideas that are standartisable. All the projects will be of an experimental nature with low-maintanace and support. The projects here shouldn't be used for production before being peer-review and explicitly green-lighted by the team. (in case where the usage is licensed )

### Disclaimer
As part of our mission of promoting public goods and open source culture we are planning to be contributing the standards that gain adoption into the original [repository](https://github.com/CosmWasm/cw-plus) maintained by the official team working on CosmWasm. Altenatively the team might choose to maintain a standard ourselves and release it under permissive open sourse license. 

Until then everything in this repository is treated as source available only unless a project comes with a separate License file. The exact license for the whole repository is to be specified


