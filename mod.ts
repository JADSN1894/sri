//* Executes after build
//* 1. Export the function to the package
//* 2. Read the content .js and .css files at dist project folder
//* 3. Obtain the sha-[256 | 512] of the files
//* 4. Update the scripts and links of the index.html

//* Arguments
//* 1. Airtifact folder
//* 2. Encoding algorithm

export function subresourceIntegrity(): string {
    console.log("subresourceIntegrity()");
    return "Hello"
}
