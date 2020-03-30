const path = require("path");

module.exports = {
  entry: { 
    main: "./src/index.js",
    vendor: "./src/vendor.js"
  },
  // note: check html webpack plugin docs for use with ejs
  plugins: [],
  module: {
    rules: [
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