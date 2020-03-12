
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
*/
export class StepResult {

    static __wrap(ptr) {
        const obj = Object.create(StepResult.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_stepresult_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get all_known() {
        var ret = wasm.__wbg_get_stepresult_all_known(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set all_known(arg0) {
        wasm.__wbg_set_stepresult_all_known(this.ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get new_colors() {
        var ret = wasm.__wbg_get_stepresult_new_colors(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set new_colors(arg0) {
        wasm.__wbg_set_stepresult_new_colors(this.ptr, arg0);
    }
}
/**
*/
export class Wasmbrot {

    static __wrap(ptr) {
        const obj = Object.create(Wasmbrot.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_wasmbrot_free(ptr);
    }
    /**
    * @param {number} multi
    * @param {boolean} burning
    * @param {number | undefined} julia_re
    * @param {number | undefined} julia_im
    * @param {number} escape
    * @param {number} width
    * @param {number} height
    * @param {number} left
    * @param {number} top
    * @param {number} pixel_width
    * @param {number} pixel_height
    * @returns {Wasmbrot}
    */
    static new(multi, burning, julia_re, julia_im, escape, width, height, left, top, pixel_width, pixel_height) {
        var ret = wasm.wasmbrot_new(multi, burning, !isLikeNone(julia_re), isLikeNone(julia_re) ? 0 : julia_re, !isLikeNone(julia_im), isLikeNone(julia_im) ? 0 : julia_im, escape, width, height, left, top, pixel_width, pixel_height);
        return Wasmbrot.__wrap(ret);
    }
    /**
    * @param {number} multi
    * @param {boolean} burning
    * @param {number | undefined} julia_re
    * @param {number | undefined} julia_im
    * @param {number} escape
    * @param {number} left
    * @param {number} top
    * @param {number} pixel_width
    * @param {number} pixel_height
    */
    reparam(multi, burning, julia_re, julia_im, escape, left, top, pixel_width, pixel_height) {
        wasm.wasmbrot_reparam(this.ptr, multi, burning, !isLikeNone(julia_re), isLikeNone(julia_re) ? 0 : julia_re, !isLikeNone(julia_im), isLikeNone(julia_im) ? 0 : julia_im, escape, left, top, pixel_width, pixel_height);
    }
    /**
    * @param {BigInt} step_size
    * @returns {StepResult}
    */
    step(step_size) {
        uint64CvtShim[0] = step_size;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.wasmbrot_step(this.ptr, low0, high0);
        return StepResult.__wrap(ret);
    }
    /**
    * @param {number} color_dist
    */
    colorize(color_dist) {
        wasm.wasmbrot_colorize(this.ptr, color_dist);
    }
    /**
    * @returns {BigInt}
    */
    dwell() {
        wasm.wasmbrot_dwell(8, this.ptr);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getInt32Memory0()[8 / 4 + 1];
        u32CvtShim[0] = r0;
        u32CvtShim[1] = r1;
        const n0 = uint64CvtShim[0];
        return n0;
    }
    /**
    * @returns {number}
    */
    colors() {
        var ret = wasm.wasmbrot_colors(this.ptr);
        return ret;
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

