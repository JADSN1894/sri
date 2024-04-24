# List all commands
list:
    just --list

# Clean dist-sri
clean-sri:
    rm -rfv dist-sri

# Run sri in dev mode
debug-sri: clean-sri
    @wasm-pack build --target deno --out-name=index --no-pack --mode=force --dev --out-dir=../dist-sri ./sri
    
release-sri:
    @wasm-pack build --target deno --no-pack --mode force --release --out-name index --out-dir ../../dist-sri ./src/sri

# Clean
clean:
    @rm -rf dist node_modules package-lock.json 
    @npm cache clean target --force

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