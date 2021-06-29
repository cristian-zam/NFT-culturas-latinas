import React, { Component } from "react";
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import getIcons from "./icons";

//components
import Navbar from "./components/Navbar.component";
import Footer from "./components/Footer.component";
//views
import Landing from "./views/Landing.view";
import Galeria from "./views/Galeria.view";
import Detail from "./views/Detail.view";
import Mint from "./views/mintNft.view";

import notFound from "./views/notFound.view";
import MetamaskProtectedRoute from "./HOCS/MetamaskProtectedRoute.hoc";

const iconList = getIcons();
const blockListArr = [];

Object.entries(iconList).forEach(([type, icons]) => {
  Object.keys(icons).map((name) => blockListArr.push(`${name},${type}`));
});

const themeList = [
  "indigo",
  "yellow",
  "red",
  "purple",
  "pink",
  "blue",
  "green",
];

class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      darkMode: false,

      theme: "blue",
    };
  }

  render() {
    return (
      <>
        <Router>
          <Navbar theme={this.state.theme} />
          <Switch>
            <Route exact path="/" component={Landing} />
            <MetamaskProtectedRoute path="/galeria" component={Galeria} />

            <MetamaskProtectedRoute
              path="/detail/:tokenid"
              component={Detail}
            />
            <MetamaskProtectedRoute path="/minar" component={Mint} />
            <Route component={notFound} />
          </Switch>
          <Footer theme={this.state.theme} />
        </Router>
      </>
    );
  }
}

export default App;
