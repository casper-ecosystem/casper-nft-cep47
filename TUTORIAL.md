# The NFT Standard on Casper (CEP-47)

This tutorial takes you through the standard of implementing [non-fungible tokens](https://docs.casperlabs.io/glossary/N/#non-fungible-token) on the Casper Network.

## Casper NFT (CEP-47) Functions

The CEP-47 standard contains the following functions to enable NFTs.  

- [*name*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L44-L47) - Returns the name of the NFT contract
- [*symbol*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L50-L53) - Returns the symbol of the NFT contract
- [*meta*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L56-L59) - Returns the metadata of the NFT contract
- [*total_supply*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L62-L65)- Returns the amount of issued NFTs
- [*balance_of*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L68-L72) - Returns the amount of NFT tokens the `owner` holds
- [*get_token_by_index*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L75-L80) - Retrieves the NFT token at a specific index
- [*owner_of*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L83-L87) - Retrieves the owner of a given token
- [*token_meta*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L90-L94) - Retrieves the metadata for a given token
- [*update_token_meta*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L97-L103) -  A function to update the metadata of a token
- [*mint*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L106-L113) - Creates a list of tokens for a specific recipient, given the token IDs and their metadata, paired in order
- [*mint_copies*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L116-L124) - For a given address, this function creates several tokens with specific IDs but with the same metadata
- [*burn*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L127-L133) - Destroys the given tokens in the account given
- [*transfer*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L136-L142) - Transfers tokens to another account
- [*transfer_from*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L145-L152) - Transfer tokens from a given account to another account
- [*approve*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L155-L161) - Gives another account the right to transfer tokens from this account
- [*get_approved*](https://github.com/casper-ecosystem/casper-nft-cep47/blob/09b40b0caf4cfc6f73d1e5f7d5b9c868228f7621/cep47/bin/cep47_token.rs#L164-L169) - Retrieves information about the rights to transfer tokens from another account

**Note**:
These functions can only be called from inside contracts, as they return data.
- *name*, *symbol*, *meta*, and *total_supply*: Return details regarding the whole contract
- *balance_of* and *get_token_by_index*: Retrieve details of tokens related to a specific account
- *owner_of* and *token_meta*: Retrieve the details of a specific token


# Preparation

First clone the contract from GitHub:

```bash
git clone https://github.com/casper-ecosystem/casper-nft-cep47.git
```

Then, move to cloned folder and prepare your environment with the following command:

```bash
cd casper-nft-cep47
make prepare
```

If your environment is set up correctly, you will see this output:

```bash
rustup target add wasm32-unknown-unknown
info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
```

If you do not see this message, check the [getting started guide](https://docs.casperlabs.io/dapp-dev-guide/getting-started/).

Next, compile your contract and run the contract unit tests.

```bash
make build-contract
make test
```
# Implementation

This section will explore a smart contract that implements the NFT standard for the Casper Network, introduced as CEP-47. Please visit [GitHub](https://github.com/casper-ecosystem/casper-nft-cep47) for the most up-to-date implementation.

To successfully execute this reference contract, you must copy the entire [contract file](https://github.com/casper-ecosystem/casper-nft-cep47/blob/master/cep47/bin/cep47_token.rs) with all the necessary imports, declarations, and functions. To execute the contract, you need to deploy the .wasm file on the network.

## Installing Required Crates
This sample CEP-47 NFT contract requires the following crates to function correctly:
- [casper_contract](https://docs.rs/casper-contract/latest/casper_contract/) - A Rust library for writing smart contracts on the Casper Network
- [casper_types](https://docs.rs/casper-types/latest/casper_types/) - Types used to allow the creation of Wasm contracts and tests for use on the Casper Network
- cep47 - A library for developing CEP-47 tokens for the Casper Network

<img src="./images/crate_imports.png" alt="import-crates" title="import-crates" width="600">

## Constructing the Contract    
The constructor uses three arguments to initialize the contract:
- `name` - Name of the NFT token 
- `symbol` - Symbol of the NFT token 
- `meta` - Metadata about the NFT token

<img src="./images/cep47_constructor.png" alt="constructor-args" title="import-crates" width="600">


## Implementing Contract Endpoints
Contract endpoints handle token operations with your account and other accounts. Refer to the list of [endpoints](#casper-nft-cep-47-functions) in the introduction section and [endpoint event stream](#contract-interaction) details in the token management section.

# Deployment

Now that you have implemented a smart contract for CEP-47, it's time to deploy it to the network. You will use a JavaScript client with in-built TypeScript (TS) classes to execute the contract deployment. The JS client also resides in a separate repository. Clone that repository to your machine and proceed with these deployment steps.

## Prerequisites
- Set up your machine as per the [prerequisites](https://docs.casperlabs.io/workflow/setup/)
- Ensure you have [set up an account](https://docs.casperlabs.io/workflow/setup/#setting-up-an-account) with a public and secret key pair to initiate the deploy
- Since we are deploying to the Casper Testnet, ensure your [Testnet faucet account](https://testnet.cspr.live/tools/faucet) contains enough CSPR tokens to perform the contract execution. Follow [fund your account](https://docs.casperlabs.io//workflow/setup#fund-your-account) guide to add CSPR tokens to your account
- CSPR tokens are used to pay for the transactions on the Casper Network. Follow the [transfer tokens](https://docs.casperlabs.io//workflow/token-transfer#2-the-faucet) guide to learn more about token transferring on the Casper Testnet

## Basic Flows of the Deployment
Here are the basic steps for deploying your contract on the Casper Network.

<img src="./images/CEP-47-deploy-flow.png" alt="deploy-flow" title="import-crates" width="600">

### Casper Repositories

You will be using two Casper repositories for the deployment process.
-  [casper-nft-cep47](https://github.com/casper-ecosystem/casper-nft-cep47) - a repository containing the implementation of the NFT, a.k.a. CEP-47 smart contract, required utility classes, and corresponding test suite to work with the CEP-47 token.
    - You will be using the *cep47-token.wasm* file from this repository for the deployment. The .wasm file is the compiled implementation of the CEP-47 contract.
- [casper-contracts-js-clients](https://github.com/casper-network/casper-contracts-js-clients) - a repository containing a JS client for the CEP-47 contract and other supporting classes to run the client. 
    - You will be executing the [install.ts](https://github.com/casper-network/casper-contracts-js-clients/blob/master/e2e/cep47/install.ts) file for the deployment.

## Deploying the Contract

### 1. Preparing the CEP-47 contract repository

Refer to the [contract preparation](#prepare) step to prepare the [NFT contract](#casper-nft-cep47) for deployment. This step will make the build environment, create the target location and compile the contract to a .wasm file.

- Output from this would be a Wasm file (Eg: *cep47-token.wasm), which is later used by the JS compiler for contract deployment.


### 2. Preparing the JS client repository
The JS client can be used to install the smart contract on the Casper Network and perform further actions with the contract. We are using the JS client classes to invoke the NFT installation on the network using a pre-defined set of environment variables.

Clone the [casper-contracts-js-clients](https://github.com/casper-network/casper-contracts-js-clients) repository using the following command.

```
git clone https://github.com/casper-network/casper-contracts-js-clients.git
```

### 3.  Adding the environment variables 

1. In the root folder of the newly-cloned repository, copy or rename the sample .env file from *.env.cep47.example* to *.env.cep47*:
  ```bash
    cd casper-contracts-js-clients
    cp .env.cep47.example .env.cep47
  ```

2. In the *.env.cep47* file, replace the following values with your own:
    - `WASM_PATH` - Path to the compiled *cep47-token.wasm* file
    - `CHAIN_NAME` - Network name, e.g. *casper* for Mainnet or *casper-test* for Testnet
    - `NODE_ADDRESS ` - Address of the node's JSON-RPC server. Use port 7777 or whatever is specified as the *rpc_server.address* in the node's config.toml, and append */rpc* to the path. Example: 195.201.174.222:7777/rpc
    - `EVENT_STREAM_ADDRESS`: Address of the node's event stream server. Use port 9999 or whatever is specified as the *event_stream_server.address* in the node's config.toml, and append  */events/main* to the path. Example: 195.201.174.222:9999/events/main
    - `MASTER_KEY_PAIR_PATH` - Path to the generated key pair for your signature
    - `USER_KEY_PAIR_PATH` - Path to the generated key pair, which in this case would be the same as your `MASTER_KEY_PAIR_PATH` (In certain test scenarios, this could be a different key pair)

**Note**
You must update the above list of parameters to align with your working environment. If you need an IP address for a node on the network, [follow this guide](https://docs.casperlabs.io/workflow/setup/#acquire-node-address-from-network-peers).


### 4.  Building the JS client
Run the following commands to install the dependencies and build the client:
```bash
npm install
npm run dist
```

### 5. Deploying the contract
Run the following command to deploy and execute the CEP-47 installer. The command executes the *./e2e/cep47/install.ts* file.

```bash
npm run e2e:cep47:install
```
 
A Successful command execution produces similar output as below.

<details>
<summary>Console output for contract installation </summary>

```bash
... Contract installation deployHash: 0dcef7e7bddbc5a666aff1afbc03cf4797e3736c71fe05aee9944a26c4eeefab
... Contract installed successfully.
... Account Info:
{
  "_accountHash": "account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803",
  "namedKeys": [
    {
      "name": "bdk_nft_contract_contract_hash",
      "key": "hash-a47d35d835a5fa8a1bcd55a4426dc14e21da9b876c1617742f18813737a4ece0"
    },
    {
      "name": "bdk_nft_contract_contract_hash_wrapped",
      "key": "uref-ff9b562d357d9a258acb2b3798f82c6ec5db49a8852e2e96b0ed4b1faf873206-007"
    },
    {
      "name": "contract_package_hash",
      "key": "hash-2468facdc9a6f324f8442584fd46d911e3ac9b434dfa79435567bf71f9b8bd23"
    }
  ],
  "mainPurse": "uref-a33e25cb1e6baa38e8306dba0492183c65cb41db3dbe8f69546868a4c0cfd0d9-007",
  "associatedKeys": [
    {
      "accountHash": "account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803",
      "weight": 1
    }
  ],
  "actionThresholds": {
    "deployment": 1,
    "keyManagement": 1
  }
}
... Contract Hash: hash-a47d35d835a5fa8a1bcd55a4426dc14e21da9b876c1617742f18813737a4ece0

```

</details>


### 6.  Installing the contract
This section clarifies how the contract deployment happens through the [install.ts](https://github.com/casper-network/casper-contracts-js-clients/blob/master/e2e/cep47/install.ts) file.

Firstly, the client reads the contents of the .wasm file into the `getBinary` constant.
```javascript
export const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};
```

Then, it creates the token metadata fetched from the *.env.cep47* file.

```javascript
const TOKEN_META = new Map(parseTokenMeta(process.env.TOKEN_META!));
```
It also fetches the keys for signing from the .env.cep47 file.

```javascript
const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);
```

Then, it fetches the node address and chain name of the network being targeted.

```javascript
const test = async () => {
  const cep47 = new CEP47Client(
    NODE_ADDRESS!,
    CHAIN_NAME!
  ); 
```
Next, it runs the installer by calling `cep47.install()`. This function takes the Wasm file path, token metadata, payment amount, and keys as parameters. The result is stored in `installDeployHash`.


```javascript
const installDeployHash = await cep47.install(
    getBinary(WASM_PATH!),
    {
      name: TOKEN_NAME!,
      contractName: CONTRACT_NAME!,
      symbol: TOKEN_SYMBOL!,
      meta: TOKEN_META
    },
    INSTALL_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS],
  );
```

Then the generated installation deploy hash is sent to the node address that you specified in the .env file. At this point, you can see the "... Contract installation deployHash: " message on the console output.

```javascript
const hash = await installDeployHash.send(NODE_ADDRESS!);
```

After that, check if the deploy is successful and retrieve the account information using the node address and public key. Next, you can see the "Contract installed successfully." message on the console.

```javascript
await getDeploy(NODE_ADDRESS!, hash)
let accountInfo = await getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);
```

Finally, the contract hash is derived from account information, and you can check the installed contract hash on the console.
```javascript
 const contractHash = await getAccountNamedKeyValue(
    accountInfo,
    `${CONTRACT_NAME!}_contract_hash`
  );
```
# Contract Interaction and Events

The NFT contract emits events. These events occur when some operation (like minting token) succeeds.

There are seven main event types for Casper CEP-47 token contract. Those are:
- [Mint](#minting-tokens)
- [Burn](#burning-tokens)
- [Mint Copies](#minting-copies-of-tokens)
- [Transfer](#transferring-tokens)
- [Approve](#approving-tokens)
- [Transfer From](#transferring-tokens-from-another-account)
- [Update Metadata](#updating-token-metadata)

We will go through each one with examples in the next sections. 

**Prerequisite**

Make sure you have [installed the CEP-47 contract](deploy.md) on the Casper Network.

## Enabling the Event Stream
To trigger the events related to the contract, you must run the *casper-contracts-js-clients/e2e/cep47/usage.ts* file using NodeJS. 

This is the command to run the file:
```bash
npm run e2e:cep47:usage
```

You will see the output as below:

<details>
<summary>Console output for deploying the token event stream</summary>

```bash
... Account Info:
{
  "_accountHash": "account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803",
  "namedKeys": [
    {
      "name": "bdk_nft_contract_contract_hash",
      "key": "hash-a47d35d835a5fa8a1bcd55a4426dc14e21da9b876c1617742f18813737a4ece0"
    },
    {
      "name": "bdk_nft_contract_contract_hash_wrapped",
      "key": "uref-ff9b562d357d9a258acb2b3798f82c6ec5db49a8852e2e96b0ed4b1faf873206-007"
    },
    {
      "name": "contract_package_hash",
      "key": "hash-2468facdc9a6f324f8442584fd46d911e3ac9b434dfa79435567bf71f9b8bd23"
    }
  ],
  "mainPurse": "uref-a33e25cb1e6baa38e8306dba0492183c65cb41db3dbe8f69546868a4c0cfd0d9-007",
  "associatedKeys": [
    {
      "accountHash": "account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803",
      "weight": 1
    }
  ],
  "actionThresholds": {
    "deployment": 1,
    "keyManagement": 1
  }
}
... Contract Hash: hash-a47d35d835a5fa8a1bcd55a4426dc14e21da9b876c1617742f18813737a4ece0
... Contract Package Hash: hash-2468facdc9a6f324f8442584fd46d911e3ac9b434dfa79435567bf71f9b8bd23
... Contract name: bdk_nft_token
... Contract symbol: BDK47
... Contract meta: [[{"isCLValue":true,"data":"1"},{"isCLValue":true,"data":"bdk-nft-1"}],[{"isCLValue":true,"data":"2"},{"isCLValue":true,"data":"bdk-nft-2"}]]
... Total supply: 0
```

</details>

The *casper-cep47-js-clients* provides a `CEP47EventsParser` which can be used in combination with JS-SDK’s [EventStream](https://github.com/casper-ecosystem/casper-js-sdk/blob/master/src/services/EventStream.ts#L73-L141).

Example code:
```rust
const es = new EventStream(EVENT_STREAM_ADDRESS!);

es.subscribe(EventName.DeployProcessed, (event) => {
  const parsedEvents = CEP47EventParser({
    contractPackageHash, 
    eventNames: [
      CEP47Events.MintOne,
      CEP47Events.TransferToken,
      CEP47Events.BurnOne,
      CEP47Events.MetadataUpdate,
      CEP47Events.ApproveToken
    ]
  }, event);

  if (parsedEvents && parsedEvents.success) {
    console.log("*** EVENT ***");
    console.log(parsedEvents.data);
    console.log("*** ***");
  }
});

es.start();
```

### Minting tokens

The token minting process creates NFTs. The Casper virtual machine executes the code stored in the smart contract and maps the item to a blockchain token containing certain attributes known as metadata. The creator's public key serves as a certificate of authenticity for that particular NFT.

#### 1. Minting the token
The `mint` method requires input parameters like recipient address, token ID, token metadata, and the payment amount to generate the NFT token. The list of input parameters is specified in the *.env.cep47* file and can be customized for each NFT implementation. This method will execute those parameters and generate the deploy object as `mintDeploy`. Then that deploy object is sent to the network via the node address to get the `mintDeployHash`. The console will output the deploy hash. Then when minting got confirmed through event stream - name of the event, CL values, and the token mint successful message will be printed.

The code snippet below is executing the [mint](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L123-L130) method. In this example, a token with ID 1 is minted with the metadata *number* and *one*.


```javascript
const mintDeploy = await cep47.mint(
    KEYS.publicKey,
    ["1"],
    [new Map([['number', 'one']])],
    MINT_ONE_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );
```

#### 2. Sending the deploy to the network
Send the 'mintDeploy' to the network via the node address and get the deploy hash.
```bash
 const mintDeployHash = await mintDeploy.send(NODE_ADDRESS!);
```

#### 3. Check the account balance
After minting the token with ID 1, you can check the balance of tokens assigned to a specific public key using the `balanceOf` method. This method returns the number of tokens stored in this account.

```javascript
const balanceOf1 = await cep47.balanceOf(KEYS.publicKey);
```
#### 4. Check token ownership
You can check the token owner by calling the `getOwnerOf` method. This method takes the token ID as the input parameter and returns the prefixed account hash of the account owning this specific token. **Note**: the prefix is *account-hash-*. 

```javascript
const ownerOfTokenOne = await cep47.getOwnerOf("1");
```

#### 5. Token index and metadata
You can also check the token metadata, the index of the token, and the token ID using the methods below.

```javascript
const tokenOneMeta = await cep47.getTokenMeta("1");
```

```javascript
const indexByToken1 = await cep47.getIndexByToken(KEYS.publicKey, "1");
```

```javascript
const tokenByIndex1 = await cep47.getTokenByIndex(KEYS.publicKey, indexByToken1);
```

<details>
<summary>Console output for token minting</summary>

```bash
... Mint token one
...... Mint deploy hash:  bd6f088d9687b51edf7d0669a1153365e7a9bd2b67064762979d03a21fd7aea2
*** EVENT ***
[
  {
    name: 'cep47_mint_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token minted successfully
...... Balance of master account:  1
...... Owner of token one:  account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803
...... Token five metadata:  Map(1) { 'number' => 'one' }
...... index of token one:  0
...... token one id:  1
```

</details>

### Burning tokens
The token burning process permanently removes the tokens from circulation within the blockchain network. The tokens are sent to a wallet address called "burner" or "eater" that cannot be used for transactions other than receiving these tokens. Even though the tokens will still exist on the blockchain, there will be no way of accessing them. 

#### Executing the burn method
The code snippet below will execute when calling the [burn](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L165-L171) method.

```javascript
const burnDeploy = await cep47.burn(
    KEYS.publicKey,
    ["1"],
    MINT_ONE_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );

```

#### Sending the deploy to the network
```javascript
const burnDeployHash = await burnDeploy.send(NODE_ADDRESS!);
```

The `burn` method executes given the values passed in and generates a `burnDeploy` object. Then, the deploy is sent to the network. When the `burn` operation got confirmed by the event stream, the name of the event and corresponding CL values, and a message indicating success or failure got printed.

<details>
<summary>Console output for token burning</summary>

```bash
... Burn token one

... Burn deploy hash:  76761cc2e1b51cb2fc6e91c61adc1139c9466316fd8bf98a4f2de05b22a31b63
... Account Info:
{
  "_accountHash": "account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803",
  "namedKeys": [
    {
      "name": "bdk_nft_contract_contract_hash",
      "key": "hash-a47d35d835a5fa8a1bcd55a4426dc14e21da9b876c1617742f18813737a4ece0"
    },
    {
      "name": "bdk_nft_contract_contract_hash_wrapped",
      "key": "uref-ff9b562d357d9a258acb2b3798f82c6ec5db49a8852e2e96b0ed4b1faf873206-007"
    },
    {
      "name": "contract_package_hash",
      "key": "hash-2468facdc9a6f324f8442584fd46d911e3ac9b434dfa79435567bf71f9b8bd23"
    }
  ],
  "mainPurse": "uref-a33e25cb1e6baa38e8306dba0492183c65cb41db3dbe8f69546868a4c0cfd0d9-007",
  "associatedKeys": [
    {
      "accountHash": "account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803",
      "weight": 1
    }
  ],
  "actionThresholds": {
    "deployment": 1,
    "keyManagement": 1
  }
}
... Contract Hash: hash-a47d35d835a5fa8a1bcd55a4426dc14e21da9b876c1617742f18813737a4ece0
... Contract Package Hash: hash-2468facdc9a6f324f8442584fd46d911e3ac9b434dfa79435567bf71f9b8bd23
... Contract name: bdk_nft_token
... Contract symbol: BDK47
... Contract meta: [[{"isCLValue":true,"data":"1"},{"isCLValue":true,"data":"bdk-nft-1"}],[{"isCLValue":true,"data":"2"},{"isCLValue":true,"data":"bdk-nft-2"}]]
... Total supply: 0

*************************
*** EVENT ***
[
  {
    name: 'cep47_burn_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
... Token burned successfully
```

</details>

### Minting copies of tokens
The method `mintCopies` creates several tokens with different IDs but the same metadata. The process is the same as minting one token but with multiple IDs and metadata. The payment amount also changes accordingly.

#### Executing mintCopies
The below code snippet executes when calling the [mintCopies](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L187-L195) method.

```javascript
const mintCopiesDeploy = await cep47.mintCopies(
    KEYS.publicKey,
    ["2", "3", "4", "5"],
    new Map([['number', 'from-series']]),
    4,
    MINT_COPIES_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );
```

#### Sending the deploy to the network
```burn
const mintCopiesDeployHash = await mintCopiesDeploy.send(NODE_ADDRESS!);
```

This method takes multiple token IDs and metadata, the token count, and other general input parameters to generate the `mintCopiesDeploy` object. Then it sends the deploy to the network. Since it is a series of tokens, we will check the token balance, owner, metadata, and index.

#### Checking token balance
This method will check the balance of tokens in the master account:
```javascript
 const balanceOf2 = await cep47.balanceOf(KEYS.publicKey);
```

#### Checking the owner
This method checks the owner of the token with ID 5:
```javascript
let ownerOfTokenFive = await cep47.getOwnerOf("5");
```

#### Checking token metadata
This method checks the metadata of the token with ID 5:
```javascript
const tokenFiveMeta = await cep47.getTokenMeta("5");
```

<details>
<summary>Console output for minting copies of a token</summary>

```bash
... Mint copies #1

...... Mint deploy hash:  e1b75c38665463da71062983b7533dc0018991487ac80a4ed8b7838f5e258ab9
... Mint token one

...... Mint deploy hash:  bd6f088d9687b51edf7d0669a1153365e7a9bd2b67064762979d03a21fd7aea2
*** EVENT ***
[
  {
    name: 'cep47_mint_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token minted successfully
...... Balance of master account:  1
...... Owner of token one:  account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803
...... Token five metadata:  Map(1) { 'number' => 'one' }
...... index of token one:  0
...... token one id:  1

^[*** EVENT ***
[
  {
    name: 'cep47_mint_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  },
  {
    name: 'cep47_mint_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  },
  {
    name: 'cep47_mint_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  },
  {
    name: 'cep47_mint_one',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token minted successfully
...... Balance of master account:  4
...... Owner of token five:  account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803
...... Token five metadata:  Map(1) { 'number' => 'from-series' }
...... index of token five:  3
...... token five id:  5
```

</details>

### Transferring tokens
This method transfers NFT token(s) to other accounts. The transfer process will initiate from your account address and be sent to the selected recipient address. The recipient address will be a randomly selected account hash in this example.

#### Executing a transfer

The code snippet below executes when calling the [transfer](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L234-L235) method.


Create the recipient address from a random number and assign it to `transferOneRecipient`.
```javascript
const transferOneRecipient = CLPublicKey.fromHex("016e5ee177b4008a538d5c9df7f8beb392a890a06418e5b9729231b077df9d7215");
```

Use the token with ID 2 and the `transferOneRecipient` address along with other input parameters to generate the `transferOneDeploy` object, and send that deploy to the network. This completes the transfer event call.

```javascript
const transferOneDeploy = await cep47.transfer(
    transferOneRecipient, 
    ["2"], 
    TRANSFER_ONE_PAYMENT_AMOUNT!, 
    KEYS.publicKey, 
    [KEYS]);
```

#### Sending the deploy to the network

```javascript
 const transferOneHash = await transferOneDeploy.send(NODE_ADDRESS!);
```

Finally, check the owner of the token with ID 2. Confirm that the owner has changed from your account hash to the recipient account hash.
```javascript
ownerOfTokenTwo = await cep47.getOwnerOf("2");
```

<details>
<summary>Console output for transferring tokens</summary>

```bash
... Transfer #1

...... Owner of token "2" is account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803
...... Transfer from account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803 to account-hash-ec0125ebcf79ab482046647049a26152166a2ed260f4ac95f279c77295b55212
...... Transfer #1 deploy hash:  e52f3cc6969fcc1641b677a66ef90c54c3368e7f141b26a3f7d4a2ba939412c2
*** EVENT ***
[
  {
    name: 'cep47_transfer_token',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token transfered successfully
...... Owner of token "2" is account-hash-ec0125ebcf79ab482046647049a26152166a2ed260f4ac95f279c77295b55212
```

</details>

### Approving tokens
This method is used to hand over the token transfer capability to another account. In this example, the new owner's public key is created before the transfer. Then the new account will perform the token transfer.

#### Executing the approve method
The following code snippet will execute when calling the [approve](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L259-L267) method.

Create the `allowedAccount` recipient address using the `KEYS_USER` variable from the *.env.cep47* file. This variable indicates the new spender of the token.

```javascript
const allowedAccount = KEYS_USER!.publicKey;
```

Next, execute the `approve` method, create the `approveDeploy` object, and send it to the network. Here, the token with ID 5 will be used for approval.

```javascript
  const approveDeploy = await cep47.approve(
    allowedAccount,
    ["5"],
    MINT_ONE_PAYMENT_AMOUNT!,
    KEYS.publicKey,
    [KEYS]
  );
```
#### Sending the deploy to the network
```javascript
const approveDeployHash = await approveDeploy.send(NODE_ADDRESS!);
```

#### Checking the new account
After generating the deploy hash for the approval, you can check which account is allowed to do the approval. It will return the account hash of the account owning this specific token.
```javascript
const allowanceOfTokenFive = await cep47.getAllowance(KEYS.publicKey, "5");
```

<details>
<summary>Console output for token approval</summary>

```bash
... Approve

...... Approval deploy hash:  940868f10945325e70ba6955c8edfe047c78ad71529bac86989d056d8ca1f26c
*** EVENT ***
[
  {
    name: 'cep47_approve_token',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token approved successfully
...... Allowance of token 5 account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803
```

</details>

### Transferring tokens from another account
Here, you will transfer tokens to another account. You will use some randomly generated account addresses to check the behavior of this method.

#### Executing the transferFrom method
The following code snippet will execute when calling the [transferFrom](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L297-L302) method.

First, check the owner of the token with ID 5.

```javascript
ownerOfTokenFive = await cep47.getOwnerOf("5");
```

Then, generate the recipient address from a random number.
```javascript
const transferFromRecipient = CLPublicKey.fromHex("019548b4f31b06d1ce81ab4fd90c9a88e4a5aee9d71cac97044280905707248da4");
```
Then, generate the `transferFromDeploy` deploy object using the new recipient address and the rest of the input parameters, complete the transfer from another account process, and send it to the network. This completes the transfer-from event call.
```javascript
const transferFromDeploy = await cep47.transferFrom(
    transferFromRecipient,
    KEYS.publicKey,
    ["5"],
    TRANSFER_ONE_PAYMENT_AMOUNT!,
    KEYS_USER.publicKey, [KEYS_USER]);
```

#### Sending the deploy to the network
```javascript
const transferFromHash = await transferFromDeploy.send(NODE_ADDRESS!);
```

#### Checking the new owner
Finally, check the owner of the token with ID 5 and note that it has changed to the new recipient.
```javascript
ownerOfTokenFive = await cep47.getOwnerOf("5");
```

<details>
<summary>Console output for transferring tokens from another account</summary>

```bash
... Transfer From #1

...... Owner of token "5" is account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803
...... Transfer from account-hash-179cd876d5c74317cce9c48d718a040e6e909063d7d786de0c5c6421a09fa803 to account-hash-fc36989e547ec1eba1d8aea840ffabbcbe7d27fb249801870551160eaa014306
...... Transfer From #1 deploy hash:  3a1e3632a401af565fad0e6c131e5347392e191e3b3c1e9a6f9c467409e055a0
*** EVENT ***
[
  {
    name: 'cep47_transfer_token',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token transfered successfully
...... Owner of token "5" is account-hash-fc36989e547ec1eba1d8aea840ffabbcbe7d27fb249801870551160eaa014306
```

</details>

### Updating token metadata 
This method will update the metadata of a selected token.

#### Executing the updateTokenMeta method
The following code snippet will execute when calling the [update metadata](https://github.com/casper-network/casper-contracts-js-clients/blob/b210261ba6b772a7cb25f62f2bdf00f0f0064ed5/e2e/cep47/usage.ts#L329-L335) method.

First, check the metadata of the token with ID 4.

```javascript
let tokenFourMeta = await cep47.getTokenMeta("4");
```

Then, execute the `updateTokenMeta` method, generate the `updateMetadataDeploy` object, and send it to the network. This completes the update metadata call.

```javascript
const updateMetadataDeploy = await cep47.updateTokenMeta(
    "4",
    new Map([["name", "four"]]),
    TRANSFER_ONE_PAYMENT_AMOUNT!,
    KEYS_USER.publicKey, 
    [KEYS_USER]
  );
```

#### Sending the deploy to the network
```javascript
const updateMetadataHash = await updateMetadataDeploy.send(NODE_ADDRESS!);
```

Again, check the metadata of the token with ID 4 and confirm the data has changed.
```javascript
tokenFourMeta = await cep47.getTokenMeta("4");
```

<details>
<summary>Console output for updating token metadata</summary>

```bash
... Update metadata of token 4

...... Token 4 metadata:  Map(1) { 'number' => 'from-series' }
...... Update metadata deploy hash:  1b5d31481bb8177d798a8368e93d5f92bf34cc493bde8caf8a078d753cdd28ec
*** EVENT ***
[
  {
    name: 'cep47_metadata_update',
    clValue: t { isCLValue: true, refType: [Array], data: [Array] }
  }
]
*** ***
...... Token metadata updated successfully
...... Token 4 metadata:  Map(1) { 'name' => 'four' }

```

</details>

