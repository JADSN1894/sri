# List all commands
list:
    just --list

# Run sri in dev mode
debug-sri:
    @wasm-pack build --target nodejs --no-pack --mode force --dev --out-name index --out-dir ../../dist-sri ./src/sri

# Run sri in release mode
release-sri:
    @wasm-pack build --target nodejs --no-pack --mode force --release --out-name index --out-dir ../../dist-sri ./src/sri
