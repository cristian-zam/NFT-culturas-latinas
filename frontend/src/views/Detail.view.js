import React, { useState } from "react";
import PropTypes from "prop-types";
import { useParams, useHistory } from "react-router-dom";
import {
  syncNets,
  getSelectedAccount,
  getContract,
  fromWEItoEth,
} from "../utils/blockchain_interaction";
import Modal from "../components/modal.component";

function LightEcommerceB(props) {
  //guarda el estado de  toda la vista
  const [state, setstate] = useState();
  //guarda el estado de el modal
  const [modal, setModal] = React.useState({
    show: false,
  });
  //es el parametro de tokenid
  const { tokenid } = useParams();
  //es el historial de busqueda
  let history = useHistory();

  React.useEffect(() => {
    (async () => {
      //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
      await syncNets();

      //obtener cuantos tokens tiene el contrato
      let totalSupply = await getContract().methods.totalSupply().call();

      //si es mayor que el total de tokens
      if (parseInt(tokenid) >= parseInt(totalSupply)) {
        window.location.href = "/galeria";
      } else {
        //obtener los datos del token que se queire
        let toks = await getContract().methods.tokensData(tokenid).call();
        //obtener el dueño del contrato
        let owner = await getContract().methods.ownerOf(tokenid).call();
        //agregar el dueño y los datos del token
        setstate({
          ...state,
          tokens: toks,
          jdata: JSON.parse(toks.data),
          owner,
        });
      }
    })();
  }, []);

  async function comprar() {
    //evitar doble compra
    setstate({ ...state, btnDisabled: true });
    //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
    await syncNets();
    //la cuenta a la cual mandaremos el token
    let account = await getSelectedAccount();

    //si el dueño intenta comprar un token le decimos que no lo puede comprar
    if (state.owner.toUpperCase() == account.toUpperCase()) {
      setModal({
        show: true,
        title: "Error",
        message: "El dueño del token no puede recomparlo",
        loading: false,
        disabled: false,
        change: setModal,
      });
      //desbloquear el boton
      setstate({ ...state, btnDisabled: false });
      return;
    }

    //modal de espera
    setModal({
      show: true,
      title: "cargando",
      message: "hola como estas",
      loading: true,
      disabled: true,
      change: setModal,
    });
    //llamar el metodo de comprar
    let toks = await getContract()
      .methods.comprarNft(state.tokens.tokenID)
      .send({
        from: account,
        value: state.tokens.price,
      })
      .catch((err) => {
        return err;
      });

    //si status esta undefined o falso le mandamos el modal de error
    if (!toks.status) {
      setModal({
        show: true,
        title: "Error",
        message: "intentalo de nuevo",
        loading: false,
        disabled: false,
        change: setModal,
      });
      //desbloquear el boton
      setstate({ ...state, btnDisabled: false });
    } else {
      setModal({
        show: true,
        title: "exito",
        message: "token comprado con exito",
        loading: false,
        disabled: false,
        change: setModal,
      });
      //desbloquear el boton
      setstate({ ...state, btnDisabled: false });
    }
  }
  return (
    <section className="text-gray-600 body-font overflow-hidden">
      <div className="container px-5 py-24 mx-auto">
        <div className="lg:w-4/5 mx-auto flex flex-wrap">
          <img
            alt="ecommerce"
            className="lg:w-1/2 w-full lg:h-auto h-64 object-fill  object-fill md:object-scale-down  rounded"
            src={`https://ipfs.io/ipfs/${state?.jdata.image}`}
          />
          <div className="lg:w-1/2 w-full lg:pl-10 lg:py-6 mt-6 lg:mt-0">
            <h1 className="text-gray-900 text-3xl title-font font-medium mb-1 mb-6">
              {state?.jdata.title}
            </h1>
            <p className="leading-relaxed mt-2 mb-6 font-mono ">
              {state?.jdata.description}
            </p>
            <div
              className={`flex border-l-4 border-${props.theme}-500 py-2 px-2 my-2 bg-gray-50`}
            >
              <span className="text-gray-500">TokenId</span>
              <span className="ml-auto text-gray-900">
                {state?.tokens.tokenID}
              </span>
            </div>
            <div
              className={`flex border-l-4 border-${props.theme}-500 py-2 px-2 my-2 bg-gray-50`}
            >
              <span className="text-gray-500">OnSale</span>
              <span className="ml-auto text-gray-900">
                <span
                  className={`inline-flex items-center justify-center px-2 py-1  text-xs font-bold leading-none ${
                    state?.tokens.onSale
                      ? "text-green-100 bg-green-500"
                      : "text-red-100 bg-red-500"
                  } rounded-full`}
                >
                  {state?.tokens.onSale ? "Disponible" : "No disponible"}
                </span>
              </span>
            </div>
            <div
              className={`flex border-l-4 border-${props.theme}-500 py-2 px-2 bg-gray-50`}
            >
              <span className="text-gray-500">Owner</span>
              <span className="ml-auto text-gray-900 text-xs self-center">
                {state?.owner}
              </span>
            </div>

            <div className="flex mt-6 items-center pb-5 border-b-2 border-gray-100 mb-5"></div>
            <div className="flex">
              <span className="title-font font-medium text-2xl text-gray-900">
                $ {state?.tokens.price && fromWEItoEth(state.tokens.price)} ETH
              </span>
              <button
                className={`flex ml-auto text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded`}
                disabled={state?.btnDisabled}
                onClick={async () => {
                  comprar();
                }}
              >
                Comprar
              </button>
            </div>
          </div>
        </div>
      </div>
      <Modal {...modal} />
    </section>
  );
}

LightEcommerceB.defaultProps = {
  theme: "yellow",
};

LightEcommerceB.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightEcommerceB;
