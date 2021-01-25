var webpack = require('webpack');
var path = require('path')
var CopyWebpackPlugin = require('copy-webpack-plugin');
module.exports = function (env, webpackConfig) {
    let res = {
        //页面入口文件配置
        entry: {
            index: `./src/index`
        },
        //入口文件输出配置
        output: {
            path: path.resolve(__dirname, 'build'),
            filename: '[name].js'
        },
        //插件项
        plugins: [
            new CopyWebpackPlugin(
                {
                    patterns: [
                        {
                            from: path.resolve(__dirname, './index.html'), //定义要拷贝的源目录，必填项
                            to: path.resolve(__dirname, './build/'), //定义要拷贝到的目标目录，非必填，不填写则拷贝到打包的output输出地址中
                        },
                        // {
                        //     from: path.resolve(__dirname, '../target/wasm32-unknown-unknown/debug/html.wasm'), //定义要拷贝的源目录，必填项
                        //     to: path.resolve(__dirname, './build/'), //定义要拷贝到的目标目录，非必填，不填写则拷贝到打包的output输出地址中
                        // }
                    ],
                    options: {
                        concurrency: 100,
                    },
                }
            ),
        ],
        module: {
            //加载器配置
            rules: [
                {
                    test: /\.tsx?$/,
                    use: [
                        {
                            loader: 'ts-loader',
                            options: {
                                configFile: 'tsconfig.json',
                                happyPackMode: true,
                                transpileOnly: true
                            }
                        }
                    ]
                },
                // {
                //     test: /\.(frag|vert)$/,
                //     use: 'raw-loader'
                // }
            ]
        },
        resolve: {
            extensions: ['.ts', '.tsx', '.js', '.wasm'],
        },
        externals: {

        },
        mode: 'development',
        performance: { hints: false },
        devServer: {
            disableHostCheck: true,
            contentBase: path.join(__dirname, '../build'),
            compress: true,
            port: 8080
        },
        experiments: {
            asyncWebAssembly: true,
        }
    };
    return res;
}