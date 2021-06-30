import React from "react";
import {
  getContract,
  getSelectedAccount,
  syncNets,
} from "../utils/blockchain_interaction";

function LightEcommerceA() {
  const [Landing, setLanding] = React.useState({
    theme: "yellow",
    currency: "ETH",
  });

  React.useEffect(() => {
    (async () => {
      //primero nos aseguramos de que la red de nuestro combo sea igual a la que esta en metamask
      await syncNets();

      //la cuenta a la cual mandaremos el token
      let account = await getSelectedAccount();
      console.log(account);
      let toks = await getContract().methods.obtenerNftsEnVenta().call();
      setLanding({ ...Landing, tokens: toks.filter((tok) => tok.onSale) });
    })();
  }, []);
  return (
    <section className="text-gray-600 body-font">
      <div className="container px-5 py-24 mx-auto">
        <div className="flex flex-wrap -m-4">
          {Landing.tokens &&
            Landing.tokens.map((token, key) => {
              const tokenData = JSON.parse(token.data);
              console.log(token)
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
                        {tokenData.price + " " + Landing.currency}
                      </p>
                    </div>
                  </a>
                </div>
              );
            })}
        </div>
      </div>
    </section>
  );
}

export default LightEcommerceA;
