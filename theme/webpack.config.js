const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
  mode: "production",
  entry: {
    "highlight": "./src/highlight.js"
  },
  output: {
    path: __dirname + "/../blog/theme",
    filename: "[name].js",
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      }
    ],
  },
  optimization: {
    minimize: true,
  },
  plugins: [new MiniCssExtractPlugin({ filename: "[name].css" })],
};
