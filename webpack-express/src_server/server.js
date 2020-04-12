const express = require('express');
const multer = require('multer');
const multipart = multer();

const webpack = require('webpack');
const webpackDevMiddlware = require('webpack-dev-middleware');
const webpackHotMiddleware = require('webpack-hot-middleware');
const config = require('../webpack.config');

const app = express();
const port = 8080;

const devServerEnabled = true;

if (devServerEnabled) {
  //reload=true:Enable auto reloading when changing JS file or content
  //timeout=1000:Time from disconnecting from server to reconnecting
  config.entry.app.unshift('webpack-hot-middleware/client?reload=true&timeout=1000');
  //Add HMR plugin
  config.plugins.push(new webpack.HotModuleReplacementPlugin());

  const compiler = webpack(config);

  //Enable "webpack-dev-middleware"
  app.use(webpackDevMiddlware(compiler, {
    publicPath: config.output.publicPath
  }));

  //Enable "webpack-hot-middlware"
  app.use(webpackHotMiddleware(compiler));
}

app.use(express.static('./public'));

//API
app.post('/api/add', multipart.any(), function(req, res) {
  const firstValue = parseInt(req.body.firstValue);
  const secondValue = parseInt(req.body.secondValue);
  const sum = firstValue + secondValue;

  res.json({ sum, firstValue, secondValue });
});

app.listen(port, () => {
  console.log("server started on port: " + port);
});