declare namespace wasm_bindgen {
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
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly load: (a: number, b: number, c: number) => void;
  readonly __wbg_nurikabeapp_free: (a: number, b: number) => void;
  readonly nurikabeapp_new: () => number;
  readonly nurikabeapp_start_solver: (a: number, b: number, c: number) => void;
  readonly startup: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7f434994642ecbe9: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf3e9509e8d860f80: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
