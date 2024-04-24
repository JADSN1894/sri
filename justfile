# List all commands
list:
    just --list

# Clean dist-sri
clean-sri:
    rm -rfv dist-sri

# Clean
clean:
    @rm -rf dist node_modules package-lock.json 
    @npm cache clean target --force

# Build release
sri-release-build:
    cargo build --release -p sri --target wasm32-wasi

# Build run
release-run: sri-release-build
    wasmtime --dir=/ --dir=. ./target/wasm32-wasi/release/sri.wasm Sha512 ./dist

# Test sri
test-sri:
    @rm -rfv sriwasm/ sri.wasm
    @git clone https://github.com/JADSN1894/sriwasm
    @cp -v ./sriwasm/sri.wasm ./
    @rm -rfv sriwasm/
    @deno run -A main.ts
    # @bun run main.ts

# Show all ports LISTEN
show-listen-ports:
    @ss -apt | grep LISTEN

# Kill process by pid
kill-by-pid pid:
    @kill 9 ${pid}

# Remove git cache
remove-git-cache:
    @git rm -rf --cached .