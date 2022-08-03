/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function get_example_data(): string;
/**
* @param {string} file_input
* @param {string} opt_t
* @returns {string}
*/
export function get_site_list_as_string(file_input: string, opt_t: string): string;
/**
* @param {string} file_input
* @param {string} opt_t
* @param {string} opt_w
* @param {string} opt_b
* @param {number} opt_wi
* @returns {string}
*/
export function calc_cons_capra07(file_input: string, opt_t: string, opt_w: string, opt_b: string, opt_wi: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_example_data: (a: number) => void;
  readonly get_site_list_as_string: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calc_cons_capra07: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
