import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";

//Importamos metodos de interacción con el smartcontract
import {
  getContract,
  getSelectedAccount,
  syncNets,
} from "../utils/blockchain_interaction";

import { useHistory } from "react-router";

import Modal from "../components/modalRevender.component";

function MisTokens(props) {
  //Hooks para el manejo de estados
  const [nfts, setNfts] = useState({}); //state de los token nft
  const [modal, setModal] = useState({
    //state para la ventana modal
    show: false,
  });

  const history = useHistory();

  //Hook para el manejo de efectos
  useEffect(() => {
    (async () => {
      //Comparamos la red en el combo de metamask con la red de aurora
      await syncNets();
      let account = await getSelectedAccount();
      //obtenemos el listado de nfts
      let nftsArr = await getContract().methods.tokensOf(account).call();
      //Actualizamos el estado el componente con una propiedad que almacena los tokens nft
      setNfts({ ...nfts, nfts: nftsArr });
    })();
  }, []);

  /**
   * Función que cambia a "no disponible" un token nft que esta a la venta siempre que se sea el owner
   * @param tokenId representa el token id del nft a quitar del marketplace
   * @return void
   */
  async function quitarDelMarketplace(tokenId) {
    await syncNets();
    let account = await getSelectedAccount();
    let quitar = await getContract()
      .methods.quitarDelMarketPlace(tokenId)
      .send({
        from: account,
      })
      .catch((err) => {
        return err;
      });

    //recargar la pantalla si la transacción se ejecuto correctamente
    if (quitar.status) {
      history.go(0);
    }
  }

  return (
    <>
      <section className="text-gray-600 body-font">
        <div className="container px-5 py-24 mx-auto">
          <div className="flex flex-col text-center w-full mb-20">
            <h1 className="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">
              Mis piezas de arte NFT
            </h1>
            <p className="lg:w-2/3 mx-auto leading-relaxed text-base">
              En esta sección aparecen los token nfts que has creado o
              adquirido.
            </p>
          </div>
          <div className="flex flex-wrap -m-4">
            {/* Hacemos un map del array de nft dentro del state */}
            {nfts.nfts &&
              nfts.nfts.map((nft, key) => {
                //obtenemos la data del token nft
                const nftData = JSON.parse(nft.data);
                console.log(nft);
                return (
                  //devolvemos un card por cada token nft del usuario
                  <div className="lg:w-1/3 sm:w-1/2 p-4">
                    <div className="flex relative">
                      <img
                        alt="gallery"
                        className="absolute inset-0 w-full h-full object-cover object-center"
                        src={`https://ipfs.io/ipfs/${nftData.image}`}
                      />
                      <div className="px-8 py-10 relative z-10 w-full border-4 border-gray-200 bg-white opacity-0 hover:opacity-100">
                        <h1 className="title-font text-lg font-medium text-gray-900 mb-3">
                          {nftData.title}
                        </h1>
                        <p className="leading-relaxed">{nftData.description}</p>
                        {/* Etiqueta de token en venta */}
                        <div className="flex border-l-4 border-blue-500 py-2 px-2 my-2 bg-gray-50">
                          <span className="text-gray-500">OnSale</span>
                          <span className="ml-auto text-gray-900">
                            <span
                              className={`inline-flex items-center justify-center px-2 py-1  text-xs font-bold leading-none ${
                                nft.onSale
                                  ? "text-green-100 bg-green-500"
                                  : "text-red-100 bg-red-500"
                              } rounded-full`}
                            >
                              {nft.onSale ? "Disponible" : "No disponible"}
                            </span>
                          </span>
                        </div>
                        <br></br>
                        <h2 className="tracking-widest text-sm title-font font-medium text-blue-500 mb-1">{`Adquirido en $${nftData.price} ETH`}</h2>

                        {/* Mostramos la opción de revender o quitar del marketplace */}
                        {nft.onSale ? (
                          <button
                            className={` mt-12 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
                            onClick={async () => {
                              await quitarDelMarketplace(nft.tokenID);
                            }}
                          >
                            {" "}
                            Quitar del marketplace
                          </button>
                        ) : (
                          <button
                            className={` mt-12 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
                            onClick={() => {
                              setModal({
                                ...modal,
                                show: true,
                                tokenId: nft.tokenID,
                                title: "Revender nft",
                                message:
                                  "Ingresa el costo al cual quieres poner a la venta este NFT.",
                                buttonName: "Cancelar",
                                change: setModal,
                              });
                            }}
                          >
                            {" "}
                            Poner en venta
                          </button>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })}
          </div>
        </div>

        {/* Mandamos a llamar al modal con el state como props*/}
        <Modal {...modal} />
      </section>
    </>
  );
}

MisTokens.propTypes = {
  theme: PropTypes.string,
};

MisTokens.defaultProps = {
  theme: "yellow",
};

export default MisTokens;
