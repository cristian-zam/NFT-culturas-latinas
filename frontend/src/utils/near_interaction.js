import {
  keyStores,
  connect,
  WalletConnection,
  Contract,
  utils,
} from "near-api-js";

export const storage_byte_cost = 10000000000000000000;
//export const contract_name = "nativo.near";
//export const contract_name = "dokxo.testnet";
export const contract_name = "nativodeploy.testnet";

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
    walletUrl: "https://wallet.near.org",
    helperUrl: "https://helper.mainnet.near.org",
    explorerUrl: "https://explorer.near.org",
  },
};
//son los metodos que tenemos en el smart contract
export const methodOptions = {
  viewMethods: [
    "obtener_pagina_v3",
    "obtener_pagina_v3_auction",
    "get_token",
    "get_on_sale_toks",
    "get_on_auction_toksV2",
    "storage_byte_cost",
    "enum_get_token",
    "nft_token",
    "nft_total_supply",
    "nft_tokens_for_owner",
    "nft_supply_for_owner",
    "nft_tokens",
    "tokens_of",
    "obtener_pagina_v3_by_owner",
    
  ],
  changeMethods: [
    "update_token",
    "minar",
    "ofertar_subasta",
    "extraer_token",
    "nft_transfer_call",
    // "minar",
    "comprar_nft",
    "revender",
    "subastar_nft",
    "finalizar_subasta",
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
  return new Contract(
    wallet.account(), // the account object that is connecting
    contract_name,
    {
      ...methodOptions,
      sender: wallet.account(), // account object to initialize and sign transactions.
    }
  );
}
/**
 * convierte de nears a yoctos
 *
 * */
export function fromNearToYocto(near) {
  console.log(utils.format.parseNearAmount(near.toString()));
  return utils.format.parseNearAmount(near.toString());
}
/**
 *
 *
 * convierte de yocto a near
 */
export function fromYoctoToNear(yocto) {
  return utils.format.formatNearAmount(yocto.toString());
}
/**
 * con esta funcion obtenemos el accountid de la cartera
 * */
export async function getNearAccount() {
  // conectarse a near
  const near = await connect(config.testnet);

  // crear una wallet de
  const wallet = new WalletConnection(near);

  return wallet.getAccountId();
}

export async function signOut() {
  // conectarse a near
  const near = await connect(config.testnet);

  // crear una wallet de
  const wallet = new WalletConnection(near);
  wallet.signOut();
}
