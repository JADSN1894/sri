# Subresource Integrity (SRI)

<p style="text-align: center;">
    <a href="https://jsr.io/@jadsn/vite-plugin-sri">
        <img src="https://jsr.io/badges/@jadsn/vite-plugin-sri" alt="Version badge" />
    </a>
    <a href="https://jsr.io/@jadsn/vite-plugin-sri">
        <img src="https://jsr.io/badges/@jadsn/vite-plugin-sri/score" alt="Score badge" />
    </a>
<p>

Adds subresource integrity hashes to script and stylesheet imports from at index.html file at build time.

## Before

![Before execution](./misc/BeforeBuild.png)

## After

![After execution](./misc/AfterBuild.png)

## Usage

Install wasmtime: `curl https://wasmtime.dev/install.sh -sSf | bash` and download the `sri.wasm` compiled executing the code below:

```bash
rm -rfv sriwasm/
git clone https://github.com/JADSN1894/sriwasm
cp -v ./sriwasm/sri.wasm ./
rm -rfv sriwasm/
npm run build # Your build script
```

## Ohh!! Is not a Vite(or another bundler) then...

**Execute:** `wasmtime --dir=/ --dir=. <filename>.wasm <hash algorithm> <airtifact_folder>`

**Example:** `wasmtime --dir=/ --dir=. ./target/wasm32-wasi/release/sri.wasm Sha512 ./dist`

## Disclaimer

Given the instability of the WASI API, utilizing *Wasmtime* ensures reliable execution of the compiled *sri.wasm* file.