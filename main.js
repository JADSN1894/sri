"use strict";
//* Executes after build
//* 1. Export the function to the package
//* 2. Read the content .js and .css files at dist project folder
//* 3. Obtain the sha-[256 | 512] of the files
//* 4. Update the scripts and links of the index.html
Object.defineProperty(exports, "__esModule", { value: true });
//* Arguments
//* 1. Airtifact folder
//* 2. Encoding algorithm
var index_js_1 = require("./dist-sri/index.js");
function subresourceIntegrity() {
    var sriOutput = (0, index_js_1.sri)();
    console.log(sriOutput);
    console.log("subresourceIntegrity()");
    return "Hello";
}
subresourceIntegrity();
