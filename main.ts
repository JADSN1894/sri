//* Executes after build
//* 1. Export the function to the package
//* 2. Read the content .js and .css files at dist project folder
//* 3. Obtain the sha-[256 | 512] of the files
//* 4. Update the scripts and links of the index.html

//* Arguments
//* 1. Airtifact folder
//* 2. Encoding algorithm

import { WASI } from "node:wasi";
import { env } from "node:process";
import { readFile } from "node:fs/promises";
export type Algorithm = "Sha256" | "Sha384" | "Sha512";

export interface SriOptions {
	/**
	 * Which hashing algorithms to use when calculate the integrity hash for each
	 * asset in the manifest.
	 *
	 * @default 'Sha512'
	 */
	algorithm: Algorithm;

	/**
	 * Airtifact path
	 *
	 * @default 'Sha512'
	 */
	outDir: string;
}

function subresourceIntegrity(
	options: SriOptions = {
		algorithm: "Sha512",
		outDir: "dist",
	},
): void {
	const { algorithm, outDir } = options;
	const wasi = new WASI({
		version: "preview1",
		args: ["sri", algorithm, outDir],
		env,
		preopens: {
			"/": "/",
			".": ".",
		},
	});

	(async () => {
		const wasm = await WebAssembly.compile(await readFile("sri.wasm"));

		const instance = new WebAssembly.Instance(wasm, {
			wasi_snapshot_preview1: wasi.wasiImport,
		});
		wasi.start(instance);
	})();
}

// subresourceIntegrity();
