import React from "react";
import { useFormik } from "formik";
import * as Yup from "yup";
//importamos metodos para interactuar con el smart contract, la red de aurora y el account
import { syncNets, getContract, getSelectedAccount } from "../utils/blockchain_interaction"

import { useHistory } from 'react-router'

export default function ModalRevender(props) {

  const history = useHistory()

  //Configuramos el formulario para revender un token
  const formik = useFormik({
    initialValues: {
      price: 0,
    },
    validationSchema: Yup.object({
      price: Yup.number()
        .required("Requerido")
        .positive("El precio debe ser mayor a 0")
        .moreThan(0, "No hay tokens gratis"),
    }),
    //Metodo para el boton revender del formulario
    onSubmit: async (values) => {
      //nos aseguramos que sigamos en la red de aurora
      await syncNets()
      let account = await getSelectedAccount()
      let revender = await getContract().methods.revender(props.tokenId, values.price).send({ from: account })
        .catch(err => { console.log(err) })

      //recargar la pantalla si la transacción se ejecuto correctamente
      if (revender.status) {
        history.go(0)
      }
      console.log(props.tokenId)
      console.log(values.price)
    }
  });

  return (
    props.show && (
      <>
        <div className="  justify-center items-center flex overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none ">
          <div className="w-full md:w-6/12 my-6  rounded ">
            {/*content*/}
            <div className=" rounded-lg shadow-lg  flex flex-col  bg-white outline-none focus:outline-none">
              {/*header*/}

              <div
                className={`bg-yellow-500 flex items-start justify-center font-bold uppercase p-5 border-b border-solid border-yellowGray-200 rounded text-white`}
              >
                {props.title}
              </div>

              <div className="relative p-6 flex flex-col ">
                <div className="flex justify-center">
                  <p className=" my-4 text-center text-2xl leading-relaxed">
                    {props.message}
                  </p>
                </div>

                {/* Formulario para revender */}
                <form
                  onSubmit={formik.handleSubmit}
                  className="container mx-auto flex px-5 py-24 md:flex-row flex-col items-center"
                >
                  <div className="flex justify-between ">
                    <label
                      htmlFor="price"
                      className="leading-7 text-sm text-gray-600"
                    >
                      Precio en ETH
                    </label>
                    {formik.touched.price && formik.errors.price ? (
                      <div className="leading-7 text-sm text-red-600">
                        {formik.errors.price}
                      </div>
                    ) : null}
                  </div>

                  <input
                    type="number"
                    id="price"
                    name="price"
                    className={`border-none w-full bg-gray-100 bg-opacity-50 rounded   focus:bg-transparent  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out-${props.theme}-500 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out`}
                    {...formik.getFieldProps("price")}
                  />

                  {/* Mostramos el boton de revender si se mando la propiedadd del token id del nft */}
                  {props.tokenId && (
                    <button
                      className={`bg-yellow-500 w-min mt-3  text-white active:bg-yellow-600 font-bold uppercase text-sm px-6 py-3 rounded-full shadow hover:shadow-lg outline-none focus:outline-none  ease-linear transition-all duration-150 `}
                      type="submit"
                      disabled={props.disabled}
                    >
                      Revender
                    </button>
                  )}

                </form>
                {/* Boton de cancelar en la ventana modal */}
                <div className="flex justify-end">
                  <button
                    className={`bg-yellow-500 w-min mt-3  text-white active:bg-yellow-600 font-bold uppercase text-sm px-6 py-3 rounded-full shadow hover:shadow-lg outline-none focus:outline-none  ease-linear transition-all duration-150 `}
                    type="button"
                    disabled={props.disabled}
                    onClick={() => {
                      props.change({ show: false });
                    }}
                  >
                    {props.buttonName}
                  </button>

                </div>
              </div>
            </div>
          </div>
        </div>
        <div className="opacity-25 fixed inset-0 z-40 bg-black"></div>
      </>
    )
  );
}