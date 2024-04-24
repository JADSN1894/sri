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
	// biome-ignore lint/suspicious/noExplicitAny: no package.json
	let config: any;
	return {
		name: "vite-plugin-subresource-integrity",
		apply: "build",
		enforce: "post",

		// biome-ignore lint/suspicious/noExplicitAny: no package.json
		configResolved(resolvedConfig: any) {
			config = resolvedConfig;
		},

		closeBundle: () => {
			// const outDir = config?.build?.outDir;s

			console.log("closeBundle()");
		},
	};
}
