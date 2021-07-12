import React from "react";
import {
  fromWEItoEth,
  getContract,
  getSelectedAccount,
  syncNets,
} from "../utils/blockchain_interaction";

function LightEcommerceA() {
  const [Landing, setLanding] = React.useState({
    theme: "yellow",
    currency: "ETH",
    tokens: [],
    page: 0,
    tokensPerPage: 7,
  });

  async function getPage(pag) {
    //esta funcion nos regresa todos los tokens por que solidity no permite arreglos
    //dinamicos en memory
    let toks = await getContract()
      .methods.obtenerPaginav2(Landing.tokensPerPage, pag)
      .call();

    //asignamos y filtramos todos los tokens que estan a  la venta

    setLanding({
      ...Landing,
      tokens: toks.filter((tok) => tok.onSale),
      page: pag,
    });
  }

  React.useEffect(() => {
    (async () => {
      //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
      await syncNets();

      //la cuenta a la cual mandaremos el token
      let account = await getSelectedAccount();
      //esta funcion nos regresa todos los tokens por que solidity no permite arreglos
      //dinamicos en memory
      let toks = await getContract()
        .methods.obtenerPaginav2(Landing.tokensPerPage, Landing.page)
        .call();

      //es el numero de tokens a la venta
      let onSaleToks = await getContract().methods.nTokenOnSale.call().call();
      console.log(toks);
      console.log(onSaleToks);
      //asignamos y filtramos todos los tokens que estan a  la venta

      setLanding({
        ...Landing,
        tokens: toks.filter((tok) => tok.onSale),
        nPages: Math.ceil(onSaleToks / Landing.tokensPerPage),
      });
    })();
  }, []);
  return (
    <section className="text-gray-600 body-font">
      <div className="container px-5 py-24 mx-auto">
        {/* Arroja un mensaje si no hay tokens disponibles en venta*/}
        {!Landing.tokens.length > 0 ? (
          <p className="lg:w-2/3 mx-auto leading-relaxed text-base">
            Actualmente no hay tokens NFT disponibles.
          </p>
        ) : null}
        <div className="flex flex-wrap -m-4">
          {Landing.tokens &&
            Landing.tokens.map((token, key) => {
              //a nuestro datos le aplicamos al funcion stringify por lo cual necesitamos pasarlo
              const tokenData = JSON.parse(token.data);
              return (
                <div className="lg:w-1/4 md:w-1/2 px-2 w-full my-3" key={key}>
                  <a href={"/detail/" + token.tokenID}>
                    <div className="block relative h-48 rounded overflow-hidden">
                      <img
                        alt="ecommerce"
                        className="object-cover object-center w-full h-full block"
                        src={`https://ipfs.io/ipfs/${tokenData.image}`}
                      />
                    </div>
                    <div className="mt-4">
                      <h2 className="text-gray-900 title-font text-lg font-medium">
                        {tokenData.title}
                      </h2>
                      <p className="mt-1">
                        {fromWEItoEth(token.price) + " " + Landing.currency}
                      </p>
                    </div>
                  </a>
                </div>
              );
            })}
        </div>
        <div className="bg-white px-4 py-3 flex items-center justify-center border-t border-gray-200 sm:px-6 mt-16">
          <nav
            className="relative z-0 inline-flex rounded-md shadow-sm -space-x-px"
            aria-label="Pagination"
          >
            {Landing?.page != 0 && (
              <a
                href="#"
                className="relative inline-flex items-center px-2 py-2 rounded-l-md  border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50"
              >
                <span className="sr-only">Previous</span>
                <svg
                  className="h-5 w-5"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path
                    fillRule="evenodd"
                    d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                    clipRule="evenodd"
                  />
                </svg>
              </a>
            )}
            {[...Array(Landing?.nPages)].map((page, index) => {
              return (
                <a
                  href="#"
                  className={`bg-white ${
                    Landing.page == index
                      ? "bg-yellow-100 border-yellow-500 text-yellow-600 hover:bg-yellow-200"
                      : "border-gray-300 text-gray-500 hover:bg-gray-50"
                  }  relative inline-flex items-center px-4 py-2 text-sm font-medium`}
                  key={index}
                  onClick={async () => {
                    await getPage(index);
                  }}
                >
                  {index + 1}
                </a>
              );
            })}
          </nav>
        </div>
      </div>
    </section>
  );
}

export default LightEcommerceA;
