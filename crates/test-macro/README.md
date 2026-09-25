# komorei-test

A crate that allows for exposing tests to [komorei-test-runner](../test-runner).

This crate is based on [webassembly-test](https://github.com/matklad/webassembly-test). It functions the same, except it adds a panic hook to the start of every test. This is necessary in order to log panic information to the source stdout before a panic occurs, since the wasmer execution just gives us a runtime error without any useful information.

## Usage

First, add the following dev dependencies:

```toml
[dev-dependencies]
komorei = { version = "1", features = ["test"] } # the "test" feature disables the panic handler, allowing tests to be run
komorei-test = "1"
```

In your rust code, simply attach the `komorei_test` attribute to any testing function:

```rs
#[cfg(test)]
mod test {
	use komorei_test::komorei_test;

	#[komorei_test]
	fn test_function() {
		assert_eq!(1, 1);
	}
}
```

Additionally, the `komorei-test-runner` harness is required to run the tests. You can install it by running:

```sh
cargo install --git https://github.com/tachibana-shin/komorei-sdk.git komorei-test-runner
```

In `.cargo/config.toml`, add the following:

```toml
[target.wasm32-unknown-unknown]
runner = "komorei-test-runner"
```

Then, `cargo test` will run the tests whenever you're compiling for wasm.