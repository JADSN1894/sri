import type { ResolvedConfig } from "vite";
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

			console.log("closeBundle()");
			console.log("outDir");
			console.log(outDir);
		},
	};
}
