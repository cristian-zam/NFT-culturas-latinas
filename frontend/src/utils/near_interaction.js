import {
  keyStores,
  connect,
  WalletConnection,
  Contract,
  utils,
} from "near-api-js";

export const storage_byte_cost = 10000000000000000000;
export const contract_name = "dev-1627316245657-69022433655295";
export const config = {
  testnet: {
    networkId: "testnet",
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
  },

  mainnet: {
    networkId: "mainnet",
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: "https://rpc.mainnet.near.org",
    walletUrl: "https://wallet.mainnet.near.org",
    helperUrl: "https://helper.mainnet.near.org",
    explorerUrl: "https://explorer.mainnet.near.org",
  },
};
//son los metodos que tenemos en el smart contract
export const methodOptions = {
  viewMethods: [
    "obtener_pagina_v2",
    "nft_token",
    "nft_total_supply",
    "nft_tokens",
    "tokens_of",
    "storage_byte_cost",
    "get_on_sale_toks",
  ],
  changeMethods: [
    "minar",
    "comprar_nft",
    "revender",
    "quitar_del_market_place",
  ],
};
/**
 *hacemos el signIn con near
 */
export async function nearSignIn(URL) {
  window.near = await connect(config.testnet);
  window.wallet = new WalletConnection(window.near, "latina");

  window.wallet.requestSignIn(
    contract_name, // contract requesting access
    "Latin-Art", // optional,
    URL, //EXITO
    URL // FRACASO
  );
}

export async function isNearReady() {
  // conectarse a near
  const near = await connect(config.testnet);

  // crear una wallet
  const wallet = new WalletConnection(near);
  //esta logueado ?
  console.log(wallet.isSignedIn());
  return wallet.isSignedIn();
}

/**
 * nos regresa una instancia del contrato
 */
export async function getNearContract() {
  // conectarse a near
  const near = await connect(config.testnet);

  // crear una wallet de
  const wallet = new WalletConnection(near);

  console.log(near);
  console.log(wallet);

  return new Contract(
    wallet.account(), // the account object that is connecting
    contract_name,
    {
      ...methodOptions,
      sender: wallet.account(), // account object to initialize and sign transactions.
    }
  );
}

export function fromNearToYocto(near) {
  console.log(utils.format.parseNearAmount(near.toString()));
  return utils.format.parseNearAmount(near.toString());
}

export function fromYoctoToNear(yocto) {
  return utils.format.formatNearAmount(yocto.toString());
}
