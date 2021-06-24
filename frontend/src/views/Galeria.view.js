import React from "react";

//components

import OnSale from "../components/onSaleNftcomponent";
export default function Landing() {
  const [Landing, setLanding] = React.useState({ theme: "blue" });
  return (
    <>
      <OnSale />
    </>
  );
}
