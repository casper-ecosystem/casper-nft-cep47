const { Keys, CLPublicKey } = require("casper-js-sdk");
const { CEP47Client, utils } = require("casper-cep47-js-client");

const NODE_ADDRESS = 'http://3.143.158.19:7777/rpc';
// const NODE_ADDRESS = 'http://localhost:40101/rpc';
const INSTALL_PAYMENT_AMOUNT = '200000000000';
const MINT_ONE_PAYMENT_AMOUNT = '2000000000';
const MINT_COPIES_PAYMENT_AMOUNT = '100000000000';
const BURN_ONE_PAYMENT_AMOUNT = '2000000000';
const CHAIN_NAME = 'integration-test';
// const CHAIN_NAME = 'casper-net-1';
const WASM_PATH = "./../target/wasm32-unknown-unknown/release/dragons-nft.wasm";
const TOKEN_NAME = 'event_nft_2';
const TOKEN_SYMBOL = 'DRAG';
const TOKEN_META = new Map([
    ['origin', 'fire'], 
    ['lifetime', 'infinite']
]);
const KEY_PAIR_PATH = '/home/ziel/workspace/casperlabs/integration-key/master';
const KEYS = Keys.Ed25519.parseKeyFiles(
    `${KEY_PAIR_PATH}/public_key.pem`,
    `${KEY_PAIR_PATH}/secret_key.pem`
);
const MINT_ONE_META_SIZE = 4;
const MINT_COPIES_META_SIZE = 10;
const MINT_MANY_META_SIZE = 10;
const MINT_COPIES_COUNT = 5;
const CONTRACT_HASH = '854db6358ec33c9d5049421f141165bd373d387fb14bb1dfa3319cdc8c873597';

const cep47 = new CEP47Client(NODE_ADDRESS, CHAIN_NAME);

const install = async () => {
    const deployHash = await cep47.install(
      KEYS, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_META, INSTALL_PAYMENT_AMOUNT, WASM_PATH);
    console.log(`Contract Installed`);
    console.log(`... DeployHash: ${deployHash}`);
};

const mintOne = async () => {
    cep47.setContractHash(CONTRACT_HASH);
    let meta = randomMetaMap(MINT_ONE_META_SIZE);
    const deployHash = await cep47.mintOne(KEYS, KEYS.publicKey, meta, MINT_ONE_PAYMENT_AMOUNT);
    console.log(`Mint One`);
    console.log(`... DeployHash: ${deployHash}`);
}

const pause = async () => {
    cep47.setContractHash(CONTRACT_HASH);
    const deployHash = await cep47.pause(KEYS, MINT_ONE_PAYMENT_AMOUNT);
    console.log(`Pause`);
    console.log(`... DeployHash: ${deployHash}`);
}

const mintCopies = async () => {
    cep47.setContractHash(CONTRACT_HASH);
    let meta = randomMetaMap(MINT_COPIES_META_SIZE);
    const deployHash = await cep47.mintCopies(
        KEYS, KEYS.publicKey, meta, MINT_COPIES_COUNT, MINT_COPIES_PAYMENT_AMOUNT);
    console.log(`Mint Copies`);
    console.log(`... DeployHash: ${deployHash}`);
}

const mintMany = async () => {
    cep47.setContractHash(CONTRACT_HASH);
    let meta = randomMetaArray(MINT_MANY_META_SIZE);
    const deployHash = await cep47.mintMany(
        KEYS, KEYS.publicKey, meta, MINT_COPIES_PAYMENT_AMOUNT);
    console.log(`Mint Many`);
    console.log(`... DeployHash: ${deployHash}`);
}

const burnOne = async (tokenId) => {
    cep47.setContractHash(CONTRACT_HASH);
    const deployHash = await cep47.burnOne(KEYS, KEYS.publicKey, tokenId, BURN_ONE_PAYMENT_AMOUNT);
    console.log(`Burn One`);
    console.log(`... DeployHash: ${deployHash}`);
}

const getName = async () => {
    cep47.setContractHash(CONTRACT_HASH);
    const value = await cep47.name();
    console.log(`Contract Name: ${value}`);
}

const balanceOf = async () => {
    cep47.setContractHash(CONTRACT_HASH);
    const balance = await cep47.balanceOf(CLPublicKey.fromHex("01694a09937e05f5a60b5f56d1d108f65ae716c45879fca79fca89ec1c20e15431"));
    console.log(`Balance: ${balance.value()}`);
}

const tokensOf = async (publicKeyHex) => {
    cep47.setContractHash(CONTRACT_HASH);
    const value = await cep47.getTokensOf(CLPublicKey.fromHex(publicKeyHex));
    console.log(`Tokens: ${JSON.stringify(value, null, 2)}`);
}
const ownerOf = async (tokenId) => {
    cep47.setContractHash(CONTRACT_HASH);
    const value = await cep47.getOwnerOf(tokenId);
    console.log(`Owner public key hex: ${value}`);
}

const tokenMeta = async (tokenId) => {
    cep47.setContractHash(CONTRACT_HASH);
    const value = await cep47.getTokenMeta(tokenId);
    console.log('Token meta', value);
}

const printAccount = async () => {
    let account = await utils.getAccountInfo(NODE_ADDRESS, KEYS.publicKey);
    console.log(account);
}

const getSimpleValue = async (name) => {
    cep47.setContractHash(CONTRACT_HASH);
    const value = await cep47[name]();
    console.log(name, value);
};

const randomMetaMap = (size) => {
    let data = new Map(); 
    for (let i = 0; i < size; i++) {
        data.set(`key${i}`, `value${i}`);
    }
    return data;
}

const randomMetaArray = (size) => {
    let arr = []; 
    for (let i = 0; i < size; i++) {
      const item = randomMetaMap(3);
    }
    return arr;
}

const command = process.argv.slice(2)[0];
const arg1 = process.argv.slice(2)[1];

switch (command) {
    case 'install_contract':
        install();
        break;
    case 'mint_one':
        mintOne();
        break;
    case 'mint_copies':
        mintCopies();
        break;
    case 'mint_many':
        mintMany();
        break;
    case 'name':
        getSimpleValue("name");
        break;
    case 'symbol':
        getSimpleValue("symbol");
        break;
    case 'meta':
        getSimpleValue("meta");
        break;
    case 'is_paused':
        getSimpleValue("isPaused");
        break;
    case 'burn_one':
        burnOne(arg1);
        break;
    case 'total_supply':
        getSimpleValue("totalSupply");
        break;
    case 'balance_of':
        balanceOf();
        break;
    case 'owner_of':
        ownerOf(arg1);
        break;
    case 'get_token_meta':
        tokenMeta(arg1);
        break;
    case 'tokens_of':
        tokensOf(arg1);
        break;
    case 'pause':
        pause();
        break;
    case 'print_account':
        printAccount();
        break;
    default:
        console.log(`Command unknown ${command}`)
}
