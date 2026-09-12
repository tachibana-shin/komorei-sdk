# Debugging

Now with a general understanding of what goes into creating and running a Komorei source, we can
discuss methods for debugging issues.

## Logging

The age-old solution for debugging is a bunch of print statements. Like any standard Rust program,
you can use the `println!` macro to do this, but there are a few options for viewing the resulting
logs. The standard method is to use the "Display Logs" button in the Advanced app settings. Logs
are also shown in the app's log viewer.

However, you may also notice the "Log Server" setting above the "Display Logs" button. Komorei will
post any log messages to the provided server URL, and you can run your own server with `komorei`:

```sh
komorei logcat
```

Enter the URL given to you by the command execution in the "Log Server" input field on a device
connected to the same network and all logs should be streamed.

## Writing Tests

While Rust programs targeting WebAssembly don't support the standard Rust test runner, komorei-sdk
provides a custom test runner that simulates Komorei's environment. This doesn't provide a complete
set of the available source APIs, and there may be a few differences from the Komorei app, but it
should be enough for simple tests. To get started, install the test runner with the following
command:

```sh
cargo install --git https://github.com/komorei-sdk/komorei-sdk.git komorei-test-runner
```

And then configure the project to use the test runner in the `.cargo/config.toml` file:

```toml
[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = "komorei-test-runner"
```

Projects created with `komorei init` should already have the `config.toml` file configured and
`komorei-test` added as a dev dependency. To use this, you write tests as you normally would in
Rust, but attach `#[komorei_test]` to test functions rather than `#[test]`:

```rust,noplayground
#[cfg(test)]
mod test {
	use super::*;
	use komorei_test::komorei_test;
	
	#[komorei_test]
	fn test_js_execution() {
		use komorei::imports::js::JsContext;
		let context = JsContext::new();
		let result = context.eval("1 + 2");
		assert_eq!(result, Ok(String::from("3")));
	}
}
```

If you run into any issues using the test runner, please
[create an issue](https://github.com/komorei-sdk/komorei-sdk/issues) on the GitHub repo so we can
improve it.