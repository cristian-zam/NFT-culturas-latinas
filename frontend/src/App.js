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
      ready: false,
      darkMode: false,
      copied: false,
      sidebar: false,
      codeView: false,
      currentKeyCode: null,
      view: "desktop",
      theme: "blue",
      blockType: "Blog",
      blockName: "BlogA",
      markup: "",
      loading: true,
      drizzleState: null,
    };
  }

  componentDidMount() {
    const { drizzle } = this.props;

    // suscribirse a los cambios
    this.unsubscribe = drizzle.store.subscribe(() => {
      // cada vez que el store se actualiza obtenemos los cambios
      const drizzleState = drizzle.store.getState();

      // revisar si drizzle esta listo si lo esta acutalizamos el estado del componente
      if (drizzleState.drizzleStatus.initialized) {
        this.setState({ loading: false, drizzleState });
      }
    });
    document.addEventListener("keydown", this.keyboardNavigation);
  }

  componentWillUnmount() {
    this.unsubscribe();
  }

  render() {
    return (
      <>
        <Router>
          <Navbar theme={this.state.theme} />
          <Switch>
            <Route exact path="/" component={Landing}></Route>
            <Route path="/galeria" component={Galeria}></Route>
            <Route path="/detail" component={Detail} />
          </Switch>
          <Footer theme={this.state.theme} />
        </Router>
      </>
    );
  }
}

export default App;
