const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./src/index.js",
  // note: check html webpack plugin docs for use with ejs
  plugins: [new HtmlWebpackPlugin({
    template: "./src/template.html"
  })],
  module: {
    rules: [{
      test: /\.scss$/,
      use: [
        "style-loader", // 3. injects css into DOM
        "css-loader", // 2. turns css into commonjs
        "sass-loader" // 1. turns sass into css
      ]
    },
    {
      test: /\.html$/,
      use: [
        "html-loader" // turns img src="img" to require("img")
      ]
    },
    {
      test: /\.(svg|png|jpg|gif)$/,
      use: {
        loader: "file-loader",
        options: {
          name: "[name].[hash].[ext]",
          outputPath: "img"
        }
      }
    }
  ]}
};