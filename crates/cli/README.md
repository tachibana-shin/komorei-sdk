# komorei-cli

A command-line utility for Komorei source development and testing.

To get started, run `cargo install --git https://github.com/komorei-sdk/komorei-sdk.git komorei-cli`.

## Usage

```ignore
Usage: komorei <COMMAND>

Commands:
  package  Build and package a source
  build    Build a source list
  init     Initialize a new source
  serve    Serve a source on the local network
  verify   Verify a source is ready to be published
  logcat   Run a log server for streaming debug logs
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```