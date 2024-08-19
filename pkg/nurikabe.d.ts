/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {any}
*/
export function load(input: string): any;
/**
*/
export function startup(): void;
/**
* @param {Int32Array} input
* @returns {number}
*/
export function sum_of_squares(input: Int32Array): number;
/**
* @param {Int32Array} input
* @returns {number}
*/
export function sum_of_squares_simple(input: Int32Array): number;
/**
*/
export class NurikabeApp {
  free(): void;
/**
* @returns {NurikabeApp}
*/
  static new(): NurikabeApp;
/**
* Do work in separate thread.
* @param {any} properties
* @returns {any}
*/
  start_solver(properties: any): any;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly load: (a: number, b: number, c: number) => void;
  readonly __wbg_nurikabeapp_free: (a: number, b: number) => void;
  readonly nurikabeapp_new: () => number;
  readonly nurikabeapp_start_solver: (a: number, b: number, c: number) => void;
  readonly startup: () => void;
  readonly sum_of_squares: (a: number, b: number) => number;
  readonly sum_of_squares_simple: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hda9bb714846f1c15: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2a5decc8654b913f: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
