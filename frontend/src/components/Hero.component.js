import React from "react";
import { Link } from "react-router-dom";
import PropTypes from "prop-types";

function LightHeroE(props) {
  return (
    <section className="text-gray-600 body-font">
      <div className="container mx-auto flex px-5 py-24 md:flex-row flex-col items-center">
        <div className="lg:max-w-lg lg:w-full md:w-1/2 w-5/6 mb-10 md:mb-0">
          <img
            className="object-cover object-center rounded"
            alt="hero"
            src="https://dummyimage.com/720x600"
          />
        </div>
        <div className="lg:flex-grow md:w-1/2 lg:pl-24 md:pl-16 flex flex-col md:items-start md:text-left items-center text-center ">
          <h1 className="title-font sm:text-4xl text-3xl mb-4 font-medium text-gray-900">
            Explora,revende nfts hechos por artistas latinos
          </h1>
          <p className="mb-8 leading-relaxed">
            Todos los nfts son hechos por que representan comunidades indigenas
          </p>

          <div className="flex  justify-between">
            <Link to="/galeria">
              <button className="bg-gray-100 inline-flex py-3 px-5 rounded-lg items-center hover:bg-gray-200 focus:outline-none">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="currentColor"
                  className="w-6 h-6"
                  viewBox="0 0 24 24"
                >
                  <g>
                    <g>
                      <path d="m6.25 19.5c-1.601 0-3.025-1.025-3.542-2.551l-.035-.115c-.122-.404-.173-.744-.173-1.084v-6.818l-2.426 8.098c-.312 1.191.399 2.426 1.592 2.755l15.463 4.141c.193.05.386.074.576.074.996 0 1.906-.661 2.161-1.635l.901-2.865z" />
                    </g>
                    <path d="m9 9c1.103 0 2-.897 2-2s-.897-2-2-2-2 .897-2 2 .897 2 2 2z" />
                  </g>
                  <path d="m21.5 2h-15c-1.378 0-2.5 1.122-2.5 2.5v11c0 1.378 1.122 2.5 2.5 2.5h15c1.378 0 2.5-1.122 2.5-2.5v-11c0-1.378-1.122-2.5-2.5-2.5zm-15 2h15c.276 0 .5.224.5.5v7.099l-3.159-3.686c-.335-.393-.82-.603-1.341-.615-.518.003-1.004.233-1.336.631l-3.714 4.458-1.21-1.207c-.684-.684-1.797-.684-2.48 0l-2.76 2.759v-9.439c0-.276.224-.5.5-.5z" />
                </svg>
                <span className="ml-4 flex items-start flex-col leading-none">
                  <span className="title-font font-medium">Galeria</span>
                </span>
              </button>
            </Link>
            <button className="bg-gray-100 inline-flex py-3 px-5 rounded-lg items-center lg:ml-4 md:ml-0 ml-4 md:mt-4 mt-0 lg:mt-0 hover:bg-gray-200 focus:outline-none">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                className="w-6 h-6"
                viewBox="0 0 50 50"
              >
                <g id="Layer_1_1_">
                  <path
                    d="M5.316,24.486l2.683,16.095v5.918h34v-5.917l2.683-16.095c2.296,0.181,4.317-1.676,4.317-3.987c0-2.206-1.794-4-4-4
		s-4,1.794-4,4c0,0.821,0.254,1.611,0.712,2.271L33.48,28.01L26.716,11.1C28.075,10.452,29,9.068,29,7.501c0-2.206-1.794-4-4-4
		s-4,1.794-4,4c0,1.567,0.924,2.952,2.284,3.599L16.52,28.01l-8.231-5.238c0.458-0.66,0.712-1.45,0.712-2.271c0-2.206-1.794-4-4-4
		s-4,1.794-4,4C0.999,22.812,3.008,24.67,5.316,24.486z M9.999,44.499v-3h30.01l-0.01,3H9.999z M44.999,18.499c1.103,0,2,0.897,2,2
		s-0.897,2-2,2c-0.255,0-0.508-0.05-0.752-0.15l-0.202-0.082l-0.188-0.132c-0.537-0.376-0.858-0.988-0.858-1.636
		C42.999,19.396,43.896,18.499,44.999,18.499z M22.999,7.499c0-1.103,0.897-2,2-2s2,0.897,2,2c0,0.994-0.75,1.841-1.744,1.97
		l-0.256,0.033l-0.256-0.033C23.749,9.34,22.999,8.493,22.999,7.499z M17.479,30.99l7.52-18.798l7.52,18.798l10.125-6.443
		l-2.492,14.952H9.833L7.354,24.548L17.479,30.99z M4.999,18.499c1.103,0,2,0.897,2,2c0,0.648-0.321,1.26-0.858,1.636l-0.188,0.132
		l-0.202,0.082c-0.244,0.1-0.498,0.15-0.752,0.15c-1.103,0-2-0.897-2-2S3.896,18.499,4.999,18.499z"
                  />
                  <rect x="23.999" y="34.499" width="2" height="2" />
                </g>
              </svg>
              <span className="ml-4 flex items-start flex-col leading-none">
                <span className="title-font font-medium">Top 10</span>
              </span>
            </button>
          </div>
        </div>
      </div>
    </section>
  );
}

LightHeroE.defaultProps = {
  theme: "indigo",
};

LightHeroE.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightHeroE;
