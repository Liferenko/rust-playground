const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
	entry: './index.cjs',
	output: {
		path: path.resolve(__dirname, 'dist'),
		filename: 'index.cjs',
	},
	plugins: [
		new HtmlWebpackPlugin(),
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, ".")
		}),
	],
	mode: 'development',
	resolve: {
		extensions: ['.wasm'],
	},
	experiments: {
		asyncWebAssembly: true
	}
};
