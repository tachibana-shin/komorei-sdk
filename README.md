# Komorei Rust Source API

A Rust library and toolchain for building sources for the
[Komorei](https://github.com/git-shin/komorei-app) Android app (anime streaming, Vietnamese UI).
A fork of [aidoku-rs](https://github.com/Aidoku/aidoku-rs) adapted for anime + video streams:
`.aix` → `.krx`, `manga`/`chapter` → `anime`/`episode`, and new stream APIs.

This repo contains the following crates:
- [komorei](crates/lib): A wrapper for Komorei source libraries.
- [komorei-cli](crates/cli): A command-line utility for Komorei source development and testing.
- [komorei-test](crates/test-macro): A crate that allows for exposing tests to `komorei-test-runner`.
- [komorei-test-runner](crates/test-runner): A tool for running tests on Komorei sources via a custom source runner.

## Komorei Source Development

To get started with Komorei source development, you'll need two things: Rust and komorei-cli.

If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/).
For komorei-cli, run the following command after installing Rust:

```sh
cargo install --git https://github.com/komorei-sdk/komorei-sdk.git komorei-cli
```

Then, create a new source project by running `komorei init`.

For a more detailed guide, check out the [source development book](https://komorei-sdk.github.io/komorei-sdk/book/).