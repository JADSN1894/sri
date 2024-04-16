//* Executes after build
//* 1. Export the function to the package
//* 2. Read the content .js and .css files at dist project folder
//* 3. Obtain the sha-[256 | 512] of the files
//* 4. Update the scripts and links of the index.html

//* Arguments
//* 1. Airtifact folder
//* 2. Encoding algorithm

import init, { sri } from "./dist-sri/index.js";

type Algorithm = "sha256" | "sha384" | "sha512";

async function subresourceIntegrity(): Promise<string> {
	console.log("[FROM TS]: subresourceIntegrity()");

	const sriOutput = await init().then(() => sri("asdf"));

	console.log(sriOutput);

	return "Hello";
}

subresourceIntegrity();
