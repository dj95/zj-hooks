[private]
default:
    just --choose

# Build the wasm file
build:
  cargo build --features tracing

# Build and run the plugin
run: build
  zellij -l ./plugin-dev-workspace.kdl -s zj-hooks-dev

# Watch and run tests with nextest.
test:
  cargo watch -x "nextest run --lib"

# Lint with clippy and cargo audit.
lint:
  cargo clippy --all-features --lib
  cargo audit

# Create and push a new release version.
release version:
  cargo set-version {{version}}
  direnv exec . cargo build --release
  git commit -am "chore: bump version to v{{version}}"
  git tag -m "v{{version}}" v{{version}}
  git push origin main
  git push origin "v{{version}}"
