import type { Plugin, ResolvedConfig } from "vite";
import { exec } from "node:child_process";

/**
 * Which hashing algorithms to use when calculate the integrity hash for each
 * asset in the manifest.
 *
 * @default 'Sha512'
 */
export type SriHashAlgorithm = "Sha256" | "Sha384" | "Sha512";

/**
 * Airtifact path
 *
 * @default 'dist'
 */
export type SriAirtifactPath = {
  airtifactPath: string;
};

/**
 * Sri argument options
 *
 * @default "{ 'Sha512', airtifactPath: { airtifactPath: 'dist' }}"
 */
export interface SriOptions {
  /**
   * Which hashing algorithms to use when calculate the integrity hash for each
   * asset in the manifest.
   *
   * @default 'Sha512'
   */
  algorithm: SriHashAlgorithm;

  /**
   * Airtifact path
   *
   * @default '{ algorithm: "Sha512", airtifactPath: { airtifactPath: "dist" }'
   */
  airtifactPath: SriAirtifactPath;
}

/**
 * SRI vite plugin
 *
 * @param options Sri argument options
 * @returns The vite Plugin type
 */
export function subresourceIntegrity(
  options: SriOptions = {
    algorithm: "Sha512",
    airtifactPath: { airtifactPath: "dist" },
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
      const outDir = config?.build?.outDir;
      executeSriAtVite({
        algorithm,
        airtifactPath: { airtifactPath: outDir },
      });
    },
  };
}

/**
 * Execute wasmtime with SriOptions at subresourceIntegrity function
 *
 * @param options Sri argument options
 * @returns Nothing
 */
function executeSriAtVite(sriOptions: SriOptions): void {
  const { algorithm, airtifactPath: { airtifactPath } } = sriOptions;
  const command =
    `wasmtime --dir=/ --dir=. ./sri.wasm ${algorithm} ${airtifactPath}`;
  exec(command, (error: string, stdout: string, stderr: string) => {
    if (error) {
      console.error(`[VITE PLUGIN - subresourceIntegrity - error]: ${error}`);
      return;
    }
    console.log(`[VITE PLUGIN - subresourceIntegrity - stdout]: ${stdout}`);
    console.error(`[VITE PLUGIN - subresourceIntegrity - stderr]:  ${stderr}`);
  });
}

// executeSriAtVite({
//   algorithm: "Sha512",
//   airtifactPath: { airtifactPath: "dist" },
// });
