# zzhack-cli
English | [中文文档](https://github.com/zzhack-stack/zzhack-cli/blob/main/README_ZH.md)

`zzhack-cli` is a Command Tool to help you quickly generate a [WASM](https://webassembly.org/) WebApp with simple configuration and zero code.It's worth mention that UI template is from [zzhack](https://github.com/zzhack-stack/zzhack), you can navigate to [Live Demo](https://www.zzhack.fun/) for real experience. 

## Quick start
[zzhack](https://github.com/zzhack-stack/zzhack) was written by Rust, thus you need to prepare the development environment for Rust for get some CLI which we need, such as `trunk`, `zzhack` etc. You can visit [Rust Book](https://doc.rust-lang.org/cargo/getting-started/installation.html) or [rust-lang.org](https://www.rust-lang.org/) for more detail about Rust installation.

After you have the [Rust](https://www.rust-lang.org/) development environment, you'll also need some Command Tools to help you building. Copy the following commands in your terminal to install them.

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk zzhack
```

Now, let's launch your first WASM WebApp!
```sh
zzhack init
zzhack serve
```

## Docs
You can use the zzhack default config by `zzhack init` for more detail of docs.

## License
MIT.