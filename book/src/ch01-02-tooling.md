## Tooling

The first step is to install Rust, if not already installed. The recommended method is to do so
using [rustup](https://rustup.rs/), which makes updating and managing your installation easy later
on.

With Rust installed, the next step is to install the proper target for compiling for WebAssembly.
You can do so with the following command:

```sh
rustup target add wasm32-unknown-unknown
```

In addition to the Rust tooling, the
[komorei cli](https://github.com/komorei-sdk/komorei-sdk/tree/main/crates/cli) provides all the other
necessary functionality for source development. To install it, run:

```sh
cargo install --git https://github.com/komorei-sdk/komorei-sdk.git komorei-cli
```