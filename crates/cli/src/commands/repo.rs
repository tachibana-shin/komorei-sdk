//! Commands for managing source repositories.
//!
//! A source repository is a self-contained collection of source crates (one
//! `sources/<lang>.<name>/` directory per source, optionally alongside shared
//! `templates/`) that gets published as a single source list — the Aidoku
//! `Aidoku-Community/sources` model. `repo build` packages every source and
//! assembles the `public/` source list (packages + icons + `index.min.json`);
//! the result can be served locally with `repo serve` or deployed to GitHub
//! Pages and added to the app as a repo URL.
use anyhow::{Context, anyhow};

/// Default source list name used when no `--name` is given.
pub const DEFAULT_NAME: &str = "Komorei Sources";

#[derive(clap::Subcommand)]
pub enum RepoCommand {
	/// Scaffold a new source repository (directory layout, CI workflows, gitignore)
	Init {
		/// Repository name (defaults to the directory name)
		#[arg(short, long)]
		name: Option<String>,
	},
	/// Build and package every source in the repository and assemble the source list
	Build {
		/// Root of the repository (defaults to `.`)
		#[arg(long, default_value = ".")]
		root: std::path::PathBuf,
		/// Output folder path
		#[arg(short, long, default_value = "public")]
		output: std::path::PathBuf,
		/// Source list name
		#[arg(short, long)]
		name: Option<String>,
	},
	/// Build the repository source list and serve it on the local network
	Serve {
		/// Root of the repository (defaults to `.`)
		#[arg(long, default_value = ".")]
		root: std::path::PathBuf,
		/// Output folder path
		#[arg(short, long, default_value = "public")]
		output: std::path::PathBuf,
		/// Port to serve on
		#[arg(short, long, default_value = "8080")]
		port: u16,
	},
	/// Verify every source package in the repository is ready to be published
	Verify {
		/// Root of the repository (defaults to `.`)
		#[arg(long, default_value = ".")]
		root: std::path::PathBuf,
	},
}

pub async fn run(command: RepoCommand) -> anyhow::Result<()> {
	match command {
		RepoCommand::Init { name } => init(name),
		RepoCommand::Build { root, output, name } => build(&root, &output, name),
		RepoCommand::Serve { root, output, port } => serve(&root, &output, port).await,
		RepoCommand::Verify { root } => verify(&root),
	}
}

/// Find all source crate directories in a repository.
///
/// Two layouts are supported:
/// * aidoku-community style: crates live under `<root>/sources/<lang>.<name>/`
/// * flat: crates live directly under `<root>` (e.g. when the repository is
///   checked out as the app's `sources/` folder, `sources/ophim/…`)
///
/// A directory counts as a source when it contains both a `Cargo.toml` and a
/// `res/source.json`. Entries are returned sorted for deterministic builds.
fn discover_source_dirs(root: &std::path::Path) -> anyhow::Result<Vec<std::path::PathBuf>> {
	let mut containers = vec![root.join("sources")];
	// only fall back to scanning the root itself when the aidoku-style
	// `sources/` container is absent (a repo root that is also a crate
	// container, like `sources/` inside the app checkout)
	if !containers[0].is_dir() {
		containers.push(root.to_path_buf());
	}

	let mut dirs = Vec::new();
	for container in &containers {
		if !container.is_dir() {
			continue;
		}
		for entry in std::fs::read_dir(container)
			.with_context(|| format!("Failed to read sources directory {}", container.display()))?
		{
			let path = entry
				.context("Failed to read entry in sources directory")?
				.path();
			if !path.is_dir() {
				continue;
			}
			if !path.join("Cargo.toml").is_file() {
				continue;
			}
			if !path.join("res").join("source.json").is_file() {
				continue;
			}
			// a `.skip` marker excludes a crate from the published list (e.g. local
			// dev fixtures that ship with the app instead)
			if path.join(".skip").is_file() {
				println!("skipping {} (marked with .skip)", path.display());
				continue;
			}
			dirs.push(path);
		}
	}
	dirs.sort();
	dirs.dedup();

	if dirs.is_empty() {
		return Err(anyhow!(
			"no sources found under {} (expected `sources/*/Cargo.toml` + `sources/*/res/source.json`)",
			root.display()
		));
	}

	Ok(dirs)
}

// package every discovered source, returning the `.krx` package paths
fn package_all(root: &std::path::Path) -> anyhow::Result<Vec<std::path::PathBuf>> {
	let dirs = discover_source_dirs(root)?;
	let mut packages = Vec::with_capacity(dirs.len());
	for dir in &dirs {
		println!("* {}", dir.display());
		let package = super::package::package_source(dir)?;
		println!("  -> {}", package.display());
		packages.push(package);
	}
	Ok(packages)
}

fn build(
	root: &std::path::Path,
	output: &std::path::Path,
	name: Option<String>,
) -> anyhow::Result<()> {
	let packages = package_all(root)?;
	super::build::run(
		packages,
		&output.to_path_buf(),
		Some(name.unwrap_or_else(|| DEFAULT_NAME.into())),
	)?;
	println!();
	println!(
		"Source list written to {}",
		output.join("index.min.json").display()
	);
	println!("Add its `index.min.json` URL as a repository in the app.");
	Ok(())
}

async fn serve(root: &std::path::Path, output: &std::path::Path, port: u16) -> anyhow::Result<()> {
	let packages = package_all(root)?;
	println!();
	super::serve::run(packages, &output.to_path_buf(), port).await
}

fn verify(root: &std::path::Path) -> anyhow::Result<()> {
	let dirs = discover_source_dirs(root)?;
	let mut packages = Vec::with_capacity(dirs.len());
	for dir in &dirs {
		let package = dir.join("package.krx");
		if !package.is_file() {
			println!("Packaging {} (missing package.krx)", dir.display());
			super::package::package_source(dir)?;
		}
		packages.push(package);
	}
	super::verify::run(packages)
}

// ── repo scaffolding ────────────────────────────────────────────────────────

// repo-level `.gitignore` (generated packages and build outputs are not committed)
const REPO_GITIGNORE: &str = "\
*.krx

public/icons/
public/sources/
public/*.json

sources/*/target
sources/*/public
templates/*/target

.idea
.DS_Store
";

// rustfmt config used across every source (matches the app/SDK tab indentation)
const REPO_RUSTFMT: &str = "\
hard_tabs = true
use_try_shorthand = true
use_field_init_shorthand = true
";

// GitHub Actions workflow: build every source on main and deploy `public/` to
// gh-pages. Replace the komorei-sdk remote with your own fork if needed.
const REPO_BUILD_WORKFLOW: &str = r#"name: Build source list
on:
  push:
    branches:
      - main
    paths:
      - "sources/**"
      - "templates/**"
      - ".github/workflows/build.yaml"

concurrency:
  group: ${{ github.workflow }}
  cancel-in-progress: true

permissions:
  contents: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry/index
            ~/.cargo/registry/cache
            ~/.cargo/git/db
            sources/**/target
          key: ${{ runner.os }}-cargo-${{ hashFiles('sources/**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      - name: Install komorei CLI
        run: |
          git clone --depth 1 https://github.com/komorei-sdk/komorei-sdk.git /tmp/komorei-sdk
          cargo install --path /tmp/komorei-sdk/crates/cli --locked

      - name: Build sources
        run: komorei repo build --root . --output public

      - name: Deploy to GitHub Pages
        uses: JamesIves/github-pages-deploy-action@v4.7.2
        with:
          branch: gh-pages
          folder: public
          git-config-name: GitHub Actions
          git-config-email: github-actions[bot]@users.noreply.github.com
          commit-message: Update source list
          single-commit: true
"#;

// GitHub Actions workflow: verify on PRs (formatting, clippy, source validation)
const REPO_PR_WORKFLOW: &str = r#"name: Check PR
on:
  pull_request:
    paths:
      - "sources/**"
      - "templates/**"
      - ".github/workflows/pr.yaml"

concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number }}
  cancel-in-progress: true

defaults:
  run:
    shell: bash

jobs:
  verify-sources:
    name: Verify sources
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install komorei CLI
        run: |
          git clone --depth 1 https://github.com/komorei-sdk/komorei-sdk.git /tmp/komorei-sdk
          cargo install --path /tmp/komorei-sdk/crates/cli --locked

      - name: Build and verify sources
        run: komorei repo verify --root .

  rust-lint:
    name: Rust formatting and Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
          targets: wasm32-unknown-unknown

      - name: Lint sources
        run: |
          set -uo pipefail
          failed=0
          for crate in sources/*; do
            [[ -f "$crate/Cargo.toml" ]] || continue
            echo "::group::Linting $crate"
            (cd "$crate" && cargo fmt --check) || failed=1
            (cd "$crate" && cargo clippy --target wasm32-unknown-unknown -- -D warnings) || failed=1
            echo "::endgroup::"
          done
          exit $failed
"#;

const REPO_README: &str = r#"# Komorei Sources

A collection of [Komorei](https://github.com/komorei-sdk/komorei-sdk) sources,
installable in the Komorei app — the same model as
[Aidoku-Community/sources](https://github.com/Aidoku-Community/sources).

Each `sources/<lang>.<name>/` directory is a self-contained Rust crate that
compiles to `wasm32-unknown-unknown` and is packaged as a `.krx` file. The repo
builds every source and assembles a single published source list.

## Layout

```
.github/workflows/   CI that builds the list and deploys it
sources/<id>/        one crate per source (Cargo.toml, src/, res/)
sources/<id>/res/    source.json + icon.png (packaged inside the .krx)
templates/           shared template crates for similar sites
public/              generated source list (gitignored, deployed by CI)
```

## Usage

On a device with Komorei installed, add the repo's `index.min.json` URL as a
repository in the app (Sources tab → add repo) to browse and install every
source in this list.

## Development

Requires `komorei` (the SDK CLI) and the `wasm32-unknown-unknown` target:

```sh
cargo install --path ../komorei-sdk/crates/cli     # or wherever the SDK lives
rustup target add wasm32-unknown-unknown
```

Scaffold a new source and build the whole repo:

```sh
komorei init sources/vi.example --name "Example" --url https://example.com --languages vi
komorei repo build    # packages every source and writes public/
komorei repo serve    # build + serve the list locally (add the printed URL to the app)
komorei repo verify   # check every packaged source is valid
```

Push to GitHub and the build workflow (`.github/workflows/build.yaml`) will
compile every source and publish `public/` to the `gh-pages` branch
automatically.

## Adding a source

See [CONTRIBUTING.md](CONTRIBUTING.md) for source checklist and conventions
(compiles without warnings, `cargo fmt`, `cargo clippy -D warnings`, JSON with
tabs and a trailing newline, conventional commits).
"#;

const REPO_CONTRIBUTING: &str = r#"# Contributing

Thank you for your interest in contributing to this source collection!

All changes are made through pull requests that are squashed on merge, using
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). Good
examples:

- `feat: add vi.ophim`
- `fix(vi.ophim): update base url`
- `feat(vi.ophim): add filtering and home listings`
- `chore: remove en.example`

## Source checklist

Before submitting, please work through the following:

- [ ] The source compiles without any warnings or errors
- [ ] `cargo fmt` has been run before submission
- [ ] `cargo clippy` outputs no lint warnings (`cargo clippy -- -D warnings`)
- [ ] All files have an additional newline at the end
- [ ] JSON files use tabs for indentation
- [ ] `komorei verify <path to package.krx>` passes

## Minimum source functionality

Sources should generally:

- Set `contentRating` on the packaged `source.json`
- Implement `ListingProvider`/home listings when the site supports them
- Implement `DeepLinkHandler` for series and episode urls, if possible
- Have filter options in the source matching the site's filters

Implementing home pages and listings is encouraged but not required for all
sources. The OPhim and demo sources in this repo are good references.

## Templates

Sites that share the same backend (WordPress themes, etc.) should reuse a
[template crate](templates/) instead of duplicating parser code. Templates
provide a `Params` struct for per-source configuration and an `Impl` trait
with the shared logic.
"#;

fn init(name: Option<String>) -> anyhow::Result<()> {
	let current_dir = std::env::current_dir().context("Failed to get current directory")?;
	let repo_name = name.unwrap_or_else(|| {
		current_dir
			.file_name()
			.map(|n| n.to_string_lossy().into_owned())
			.unwrap_or_else(|| "Komorei Sources".to_string())
	});

	// repository root is the current directory
	let root = current_dir.clone();

	// create the directory structure (never overwrite existing files)
	let write_file = |rel: &str, content: &str| -> anyhow::Result<()> {
		let path = root.join(rel);
		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent)
				.with_context(|| format!("Failed to create directory {}", parent.display()))?;
		}
		if path.exists() {
			// leave existing files alone (e.g. a README already present)
			println!("skipping {} (already exists)", rel);
			return Ok(());
		}
		std::fs::write(&path, content)
			.with_context(|| format!("Failed to write {}", path.display()))?;
		println!("  created {}", rel);
		Ok(())
	};

	println!(
		"Initializing source repository '{}' in {}",
		repo_name,
		root.display()
	);

	// directory layout
	let dirs = ["sources", "templates"];
	for dir in dirs {
		let path = root.join(dir);
		if !path.exists() {
			std::fs::create_dir_all(&path)
				.with_context(|| format!("Failed to create directory {}", path.display()))?;
			println!("  created {}/", dir);
		}
	}

	// scaffold files
	write_file(".gitignore", REPO_GITIGNORE)?;
	write_file("rustfmt.toml", REPO_RUSTFMT)?;
	write_file(
		"README.md",
		&REPO_README.replace("{{REPO_NAME}}", &repo_name),
	)?;
	write_file("CONTRIBUTING.md", REPO_CONTRIBUTING)?;
	write_file(".github/workflows/build.yaml", REPO_BUILD_WORKFLOW)?;
	write_file(".github/workflows/pr.yaml", REPO_PR_WORKFLOW)?;
	write_file(
		"templates/README.md",
		"# Templates\n\nShared template crates used by multiple sources. See\n[CONTRIBUTING.md](../CONTRIBUTING.md) for when to create one.\n",
	)?;

	println!();
	println!("Done. Next steps:");
	println!(
		"  1. komorei init sources/vi.example --name \"Example\" --url https://example.com --languages vi"
	);
	println!("  2. komorei repo build   (packages every source and writes public/)");
	println!("  3. komorei repo serve   (serve the list locally and add it to the app)");
	println!("  4. git init && git push (CI builds and deploys the list to gh-pages)");

	Ok(())
}
