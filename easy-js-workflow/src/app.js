import './scss/app.scss';

import Header from "./components/Header";
import User from "./components/User";

console.log("It works");

const app = async () => {
  document.getElementById('header').innerHTML = Header();
  document.getElementById('user').innerHTML = await User();
}

app();