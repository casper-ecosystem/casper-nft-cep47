# The NFT Standard on Casper (CEP-47)

[CEP-47](https://github.com/casper-ecosystem/casper-nft-cep47) is the NFT standard for the Casper blockchain, defining a minimum interface contract code should implement to manage, own, and trade unique tokens on the Casper Network. 

The Casper NFT standard takes full advantage of [unforgeable references](https://docs.casperlabs.io/design/uref/) to store values and manage permissions to them. It also takes advantage of other access control features (such as [groups](https://docs.casperlabs.io/glossary/G/#groups)). We recommend exploring the [main functions](/Basic-Tutorial.md#casper-nft-cep-47-functions) of the [contract](https://github.com/casper-ecosystem/casper-nft-cep47/blob/master/cep47/bin/cep47_token.rs) to understand the standard further.

The equivalent NFT standard on Ethereum is [ERC-721](https://eips.ethereum.org/EIPS/eip-721).

For more information on contract implementation and sending the contract to the network, visit the [CEP-47 Basic Tutorial](/Basic-Tutorial.md), an illustrated guide on implementing, deploying, and testing a sample Casper NFT contract.

Visit the [Contract Interaction and Events Tutorial](/Contract-Interaction-Tutorial.md) to get more details about emitting and monitoring contract events. 