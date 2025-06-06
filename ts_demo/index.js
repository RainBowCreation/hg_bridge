"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
// ✅ NO need for init() — just use functions directly
const unified_wasm_lib_1 = require("../pkg/unified_wasm_lib");
console.log((0, unified_wasm_lib_1.add)(2, 3));
console.log((0, unified_wasm_lib_1.greet)("TypeScript"));
