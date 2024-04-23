// import { instantiate } from "./lib/rs_lib.generated.js";

// const { add } = await instantiate();
// console.log(add(1, 1));

import { instantiate } from "./lib/rs_lib.generated.js";

export async function sum(a: number, b: number): Promise<number> {
	const { add } = await instantiate();
	return add(a, b);
}
