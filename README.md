# Sourcemap Decoder
[rust-sourcemap](https://github.com/getsentry/rust-sourcemap) nodejs bindings

# Install
Make sure have rust installed in your environment.
follow this link: https://www.rust-lang.org/en-US/install.html to install rust toolchains.

> 如果你是中国用户，可以使用 `curl https://sh.rustup.rs -sSf | sed "s/https:\/\/static.rust-lang.org\/rustup\/dist/https:\/\/mirrors.ustc.edu.cn\/rust-static\/rustup\/dist/g" | sh` 来安装

run `yarn add sourcemap-decoder` or `npm i sourcemap-decoder` to install

> 如果你是中国用户，在执行安装命令之前可以先执行(复制多行，粘贴到命令行，回车):

```bash
cat > $HOME/.cargo/config << EOF
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
```

# Working in Docker
There is a Official rust Dockerfile in [Github](https://github.com/rust-lang-nursery/docker-rust/blob/master/1.23.0/stretch/Dockerfile). You can follow this file to setup your Dockerfile.

> 如果你是中国用户，[Dockerfile](./Dockerfile) 是一份 Dockerfile 的样板文件，它可以在国内达到更快的 Build 速度


> If you want to use neon bindings with your NodeJS project in Docker, I suggest you `don't` install rust toolchains in your production use Docker Image, just install them in your `ci environment`, and run `yarn install --production` or `npm install --production`, then use Docker `ADD` command to copy all your files into your `production` Docker Image. And if you do that, don't forget to keep your NodeJS version in `ci environment` to be the same with the NodeJS version in `production` environment.

> 如果你需要在生产环境里面使用 `neon bindings` 相关的项目，建议 ***不要*** 在 `production` 使用的 Docker Image 内安装 Rust 相关的工具链，这样会极大增加 Docker Image 的体积，你可以在你的 ci 环境内安装 Rust 相关的工具，在执行 `yarn install --production` 或者 `npm install --production` 后将所有文件 `ADD` 到生产环境的 Docker Image 中。如果你这样做，确保 ci 运行时的 NodeJS 版本与 `production` 环境的 NodeJS 版本一致。

# Benchmark
run: `yarn benchmark` or `npm run benchmark` to see the benchmark result.

This is the benchmark result in my machine:

### Hardware info:

```
ProductName:    Mac OS X
ProductVersion: 10.13.3
BuildVersion:   17D47
Model Name: MacBook Pro
Model Identifier: MacBookPro14,2
Processor Name: Intel Core i5
Processor Speed: 3.1 GHz
Number of Processors: 1
Total Number of Cores: 2
L2 Cache (per Core): 256 KB
L3 Cache: 4 MB
Memory: 16 GB
```

```
$ node benchmark
JavaScript parse time 50794 microseconds
Rust parse time: 39 microseconds
JavaScript parse result, Source: webpack:///src/utils/logger/logger.ts, Line: 56
Rust parse result, Source: webpack:///./src/utils/logger/logger.ts, Line: 56
✨  Done in 0.33s.
```
