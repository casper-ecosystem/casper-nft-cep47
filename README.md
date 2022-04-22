# The NFT Standard on Casper (CEP-47)

[CEP-47](https://github.com/casper-ecosystem/casper-nft-cep47) is the NFT standard for the Casper blockchain, defining a minimum interface contract code should implement to manage, own, and trade unique tokens on the Casper Network. 

The Casper NFT standard takes full advantage of [unforgeable references](https://docs.casperlabs.io/design/uref/) to store values and manage permissions to them. It also takes advantage of other access control features (such as [groups](https://docs.casperlabs.io/glossary/G/#groups)). We recommend exploring the [main functions](TUTORIAL.md#casper-nft-cep-47-functions) of the [contract](https://github.com/casper-ecosystem/casper-nft-cep47/blob/master/cep47/bin/cep47_token.rs) to understand the standard further.

The equivalent NFT standard on Ethereum is [ERC-721](https://eips.ethereum.org/EIPS/eip-721).

For more information, visit the [CEP-47 Tutorial](TUTORIAL.md), an illustrated guide on implementing, deploying, and testing a sample Casper NFT contract.
