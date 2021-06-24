import React from "react";

//components

import Atributes from "../components/nftatribute.component";
export default function Landing() {
  const [Landing, setLanding] = React.useState({ theme: "blue" });
  return (
    <>
      <Atributes theme={Landing.theme} />
    </>
  );
}
