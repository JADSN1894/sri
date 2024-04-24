import type { Plugin, ResolvedConfig } from "vite";
// import { greet } from "./wasm/index.js";

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

function subresourceIntegrity(
	options: SriOptions = {
		algorithm: "Sha512",
	},
): Plugin {
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
			// const outDir = config?.build?.outDir;s
			// greet();
			// const wasi = new WASI({
			// 	version: "preview1",
			// 	args: ["sri", algorithm, outDir],
			// 	env,
			// 	preopens: {
			// 		"/": "/",
			// 		".": ".",
			// 	},
			// });
		},
	};
}

export { subresourceIntegrity as default };
