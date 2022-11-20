import * as path from "path";
import { fileURLToPath } from "url";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import CopyWebpackPlugin from "copy-webpack-plugin";
import Dotenv from "dotenv-webpack";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const distPath = path.resolve(__dirname, "./dist");

export default (_env, argv) => {
  return {
    devServer: {
      static: distPath,
      compress: argv.mode === "production",
      port: 8000,
      historyApiFallback: true,
    },
    entry: "./bootstrap.js",
    output: {
      publicPath: "/",
      path: distPath,
      filename: "main.js",
      webassemblyModuleFilename: "main.wasm",
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ["style-loader", "css-loader", "sass-loader"],
        },
        {
          test: /\.wasm$/,
          type: "webassembly/sync",
        },
      ],
    },
    plugins: [
      new Dotenv(),
      new CopyWebpackPlugin({
        patterns: [{ from: "./static", to: distPath }],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: `--no-typescript${
          argv.mode == "production" ? " --release" : ""
        }`,
      }),
    ],
    experiments: {
      syncWebAssembly: true,
    },
  };
};
