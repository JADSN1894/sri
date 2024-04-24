# sri

## Usage

Install wasmtime: `curl https://wasmtime.dev/install.sh -sSf | bash` and download the `sri.wasm` compiled executing the code below:

```bash
rm -rfv sriwasm/
git clone https://github.com/JADSN1894/sriwasm
cp -v ./sriwasm/sri.wasm ./
rm -rfv sriwasm/
npm run build # Your build script
```

## Disclaimer

Given the instability of the WASI API, utilizing Wasmtime ensures reliable execution of the compiled sri.wasm file.