import type { ResolvedConfig } from "vite";
// import { greet } from "./wasm/index.js";
import { WASI } from 'wasi';
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
}

export function subresourceIntegrity(
	options: SriOptions = {
		algorithm: "Sha512",
	},
) {
	const { algorithm } = options;
	let config: ResolvedConfig;
	return {
		name: "vite-plugin-subresource-integrity",
		apply: "build",
		enforce: "post",

		configResolved(resolvedConfig: ResolvedConfig) {
			config = resolvedConfig;
		},

		closeBundle: () => {
			const outDir = config?.build?.outDir;


			const wasi = new WASI({
				version: "preview1",
				args: ["sri", algorithm, outDir],
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

		},
	};
}
