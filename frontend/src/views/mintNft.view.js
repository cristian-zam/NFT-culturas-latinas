import React from "react";
import PropTypes from "prop-types";
import { useFormik } from "formik";
import * as Yup from "yup";

import { acceptedFormats } from "../utils/constraint";
import {
  addNetwork,
  fromETHtoWei,
  getContract,
  getSelectedAccount,
} from "../utils/blockchain_interaction";
function LightHeroE(props) {
  //este estado contiene toda la info de el componente
  const [mint, setmint] = React.useState({ file: undefined });
  const formik = useFormik({
    initialValues: {
      title: "",
      description: "",
      price: -1,
      image: "",
    },
    //validaciones
    validationSchema: Yup.object({
      title: Yup.string()
        .max(30, "Menos de 30 caracteres")
        .required("Requerido")
        .min(5, "el titulo debe longitud mayor a 5"),

      description: Yup.string()
        .max(50, "Menos de 50 caracteres")
        .required("Requerido")
        .min(30, "la descripción minimo es de 30 caracteres"),
      price: Yup.number()
        .required("Requerido")
        .positive("el precio debe ser positivo")
        .moreThan(0, "no existen nft gratis"),
      image: Yup.string().required("Requerido"),
    }),
    onSubmit: async (values) => {
      //la cuenta a la cual mandaremos el token
      let account = await getSelectedAccount();
      //los datos de la transacccion
      let token = await getContract()
        .methods.minar(
          account,
          JSON.stringify(values),
          fromETHtoWei(values.price)
        )
        .send({ from: account });

      console.log(token);
    },
  });
  function imageClick() {
    formik.setFieldTouched("image");
  }
  function imageChange(e) {
    //si selecciono un archivo
    if (e.target.files[0]) {
      //asignar imagen de preview
      setmint({
        file: URL.createObjectURL(e.target.files[0]),
      });

      //una vez que cargue el arhcivo lo mandamos a ipfs
      const reader = new FileReader();
      reader.readAsArrayBuffer(e.target.files[0]);
      reader.onloadend = async function () {
        window.ipfs.add(reader.result).then(async (result) => {
          console.log(result);
          console.log(`https://ipfs.io/ipfs/${result.path}`);
          formik.setFieldValue("image", result.path);
        });
      };
    }
  }
  return (
    <section className="text-gray-600 body-font">
      <form
        onSubmit={formik.handleSubmit}
        className="container mx-auto flex px-5 py-24 md:flex-row flex-col items-center"
      >
        <div className="lg:max-w-lg lg:w-full md:w-1/2 w-5/6 mb-10 md:mb-0 items-center relative">
          {mint?.file && (
            <img
              className="   bg-cover bg-center rounded  "
              alt="hero"
              src={mint?.file}
            />
          )}
          <label
            className={` title-font sm:text-4xl text-3xl  font-medium absolute inset-0  w-full flex flex-col items-center   rounded-lg  tracking-wide uppercase  cursor-pointer justify-center`}
          >
            <div
              className={`  my-4 title-font sm:text-4xl text-3xl w-full text-center ${
                mint?.file ? "bg-white" : ""
              }
            `}
            >
              {mint?.file ? "Cambiar " : "Subir Imagen"}
            </div>
            <input
              onChange={imageChange}
              onClick={imageClick}
              type="file"
              id="image"
              name="image"
              className={`  hidden `}
              accept={acceptedFormats}
            />
          </label>
          {formik.touched.image && formik.errors.image ? (
            <div className="flex leading-7 text-sm text-red-600 text-center mt-20 justify-center">
              {formik.errors.image}
            </div>
          ) : null}
        </div>
        <div className="lg:flex-grow md:w-1/2 lg:pl-24 md:pl-16 flex flex-col md:items-start md:text-left items-center text-center">
          <h1 className=" w-full title-font sm:text-4xl text-3xl mb-12 font-medium text-gray-900 text-center">
            Nuevo NFT
          </h1>
          <div className="flex w-full md:justify-start justify-center items-end">
            <div className="relative mr-4 lg:w-full xl:w-1/2 w-3/4">
              <div className="flex justify-between ">
                <label
                  htmlFor="title"
                  className="leading-7 text-sm text-gray-600"
                >
                  Titulo
                </label>
                {formik.touched.title && formik.errors.title ? (
                  <div className="leading-7 text-sm text-red-600">
                    {formik.errors.title}
                  </div>
                ) : null}
              </div>

              <input
                type="text"
                id="title"
                name="title"
                {...formik.getFieldProps("title")}
                className={`  w-full bg-gray-100 bg-opacity-50 rounded   focus:bg-transparent  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out `}
              />

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

              <div className="flex justify-between ">
                <label
                  htmlFor="description"
                  className="leading-7 text-sm text-gray-600"
                >
                  Descripción
                </label>
                {formik.touched.description && formik.errors.description ? (
                  <div className="leading-7 text-sm text-red-600">
                    {formik.errors.description}
                  </div>
                ) : null}
              </div>

              <textarea
                type="textarea"
                id="description"
                name="description"
                rows="2"
                {...formik.getFieldProps("description")}
                className={` resize-none border-none w-full bg-gray-100 bg-opacity-50 rounded   focus:bg-transparent  text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out${props.theme}-500 text-base outline-none text-gray-700 py-1 px-3 leading-8 transition-colors duration-200 ease-in-out`}
              />
              <button
                type="submit"
                className={` mt-12 w-full text-white bg-${props.theme}-500 border-0 py-2 px-6 focus:outline-none hover:bg-${props.theme}-600 rounded text-lg`}
                disabled={mint?.onSubmitDisabled}
              >
                Minar
              </button>
            </div>
          </div>
        </div>
      </form>
    </section>
  );
}

LightHeroE.defaultProps = {
  theme: "blue",
};

LightHeroE.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightHeroE;
