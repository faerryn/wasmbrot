/* tslint:disable */
/* eslint-disable */
export class Wasmbrot {
  free(): void;
/**
* @param {number} width 
* @param {number} height 
* @param {number} center_x 
* @param {number} center_y 
* @param {number} scale 
* @returns {Wasmbrot} 
*/
  static new(width: number, height: number, center_x: number, center_y: number, scale: number): Wasmbrot;
/**
*/
  tick(): void;
/**
*/
  colorize(): void;
/**
* @returns {number} 
*/
  depth(): number;
/**
* @returns {number} 
*/
  colors(): number;
}

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        