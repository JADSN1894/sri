import type { ResolvedConfig } from "vite";
import { exec } from "node:child_process";

export type Algorithm = "Sha256" | "Sha384" | "Sha512";

export type SriAirtifactPath = {
	airtifactPath: string
}

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
	airtifactPath: SriAirtifactPath;
}

export function subresourceIntegrity(
	options: SriOptions = {
		algorithm: "Sha512",
		airtifactPath: { airtifactPath: "dist" }
	}
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
			executeSriAtVite({
				algorithm,
				airtifactPath: { airtifactPath: outDir }
			});
		},
	};
}



function executeSriAtVite(sriOptions: SriOptions) {
	const { algorithm, airtifactPath: { airtifactPath } } = sriOptions;
	const command = `wasmtime --dir=/ --dir=. ./sri.wasm ${algorithm} ${airtifactPath}`;
	exec(command, (error, stdout, stderr) => {
		if (error) {
			console.error(`[VITE PLUGIN - subresourceIntegrity - error]: ${error}`);
			return;
		}
		console.log(`[VITE PLUGIN - subresourceIntegrity - stdout]: ${stdout}`);
		console.error(`[VITE PLUGIN - subresourceIntegrity - stderr]:  ${stderr}`);
	});
}

executeSriAtVite({
	algorithm: "Sha512",
	airtifactPath: { airtifactPath: "dist" }
});
