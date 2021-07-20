import React from "react";
import { Route } from "react-router-dom";
import { nearInit } from "../utils/near_interaction";

const NearProtectedRoute = ({ component: Component, ...rest }) => {
  nearInit();
  return (
    <>
      <Route
        {...rest}
        render={(props) => {
          return <Component {...props} />;
        }}
      />
    </>
  );
};

export default NearProtectedRoute;
