import { keyStores, connect, WalletConnection } from "near-api-js";

export const config = {
  testnet: {
    networkId: "testnet",

    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
  },

  mainnet: {
    networkId: "mainnet",

    nodeUrl: "https://rpc.mainnet.near.org",
    walletUrl: "https://wallet.mainnet.near.org",
    helperUrl: "https://helper.mainnet.near.org",
    explorerUrl: "https://explorer.mainnet.near.org",
  },
};

export async function nearInit() {
  const keyStore = new keyStores.BrowserLocalStorageKeyStore();

  console.log({ ...config.testnet, keyStore });
  const near = await connect({ ...config.testnet, keyStore });

  const wallet = new WalletConnection(near, "latina");
  console.log(wallet.isSignedIn());

  if (!wallet.isSignedIn()) {
    wallet.requestSignIn(
      "dev-1626280160013-8252228", // contract requesting access
      "Latin-Art" // optional
    );
  }

  return wallet.isSignedIn();
}
