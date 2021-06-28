import MarketPlace from "../contracts/MarketPlace.json";
import Web3 from "web3";
const { create } = require("ipfs-http-client");

/**
 * contiene todas las redes que podemos agregar
 */
var nets = [
    {
      chainId: 5777,
      data: [
        {
          chainId: "0x1691",
          chainName: "Truffle Develop",
          rpcUrls: ["http://127.0.0.1:9545/"],
          nativeCurrency: {
            name: "TRUFFLE COIN",
            symbol: "T-ETH",
            decimals: 18,
          },
          blockExplorerUrls: ["https://bscscan.com/"],
        },
      ],
    },
    {
      chainId: 1313161555,
      data: [
        {
          chainId: "0x4E454153",
          chainName: "AURORATESTNET",
          rpcUrls: ["https://testnet.aurora.dev"],
          nativeCurrency: {
            name: "AURORA COIN",
            symbol: "A-ETH",
            decimals: 18,
          },
          blockExplorerUrls: ["https://testnet.bscscan.com/"],
        },
      ],
    },
  ],
  nets = Object.assign(
    ...nets.map(({ chainId, data }) => ({ [chainId]: data }))
  );

export var nets;
/**
 * inicializar una instancia de ipfs
 */
export function init() {
  if (window.ethereum && !window.web3x) {
    //instancia de web3
    window.web3x = new Web3(window.ethereum);
    //red por default
    localStorage.setItem("network", 5777);

    //instancia de ipfs
    window.ipfs = create({
      host: "ipfs.infura.io",
      port: 5001,
      protocol: "https",
    });
  }
}

/**
 *nos permite saber si podemos usar alguna blockchain
 * @return bool falso si no cuenta con metamask
 */
export function isMetamaskInstalled() {
  return window.ethereum ? true : false;
}

/**
 * nos agrega y cambia de red
 * @param {int} id es el chainid a agregar o cambiar
 * @returns el resultado de la interaccion del usuario
 */
export async function addNetwork(id) {
  //obtener el arreglo con los datos de la red
  let networkData = nets[id];
  if (!networkData) return "no existe esa red";
  // agregar red o cambiar red
  return window.ethereum.request({
    method: "wallet_addEthereumChain",
    params: networkData,
  });
}

/**
 * nos regresa una instancia del contrato de la red actualmente seleccionada
 * @returns instancia del contrato
 */
export function getContract() {
  // sm address
  let smartContractAddress =
    MarketPlace.networks[localStorage.getItem("network")].address;
  //instantiate the contract object
  return new window.web3x.eth.Contract(MarketPlace.abi, smartContractAddress);
}

export async function getSelectedAccount() {
  //get the useraccounts
  let useraccounts = await window.ethereum.request({
    method: "eth_requestAccounts",
  });
  return useraccounts[0];
}

export function fromETHtoWei(eth) {
  return Web3.utils.toWei(eth.toString(), "ether");
}
