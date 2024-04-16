# List all commands
list:
    just --list

# Run sri in dev mode
debug-sri:
    @wasm-pack build --target nodejs --no-pack --mode force --dev --out-name index --out-dir ../../dist-sri ./src/sri

# Run sri in release mode
release-sri:
    @wasm-pack build --target nodejs --no-pack --mode force --release --out-name index --out-dir ../../dist-sri ./src/sri

# Show all ports LISTEN
show-listen-ports:
    @ss -apt | grep LISTEN

# Kill process by pid
kill-by-pid pid:
    @kill 9 ${pid}

# Remove git cache
remove-git-cache:
    @git rm -rf --cached .