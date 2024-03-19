import * as wasm from "./testcode_bg.wasm";
import { __wbg_set_wasm } from "./testcode_bg.js";
__wbg_set_wasm(wasm);
export * from "./testcode_bg.js";
