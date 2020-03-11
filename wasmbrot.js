
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
    * @param {number} width
    * @param {number} height
    * @param {number} left
    * @param {number} top
    * @param {number} pixel_size
    * @returns {Wasmbrot}
    */
    static bounds(width, height, left, top, pixel_size) {
        var ret = wasm.wasmbrot_bounds(width, height, left, top, pixel_size);
        return Wasmbrot.__wrap(ret);
    }
    /**
    * @param {number} step_size
    */
    step(step_size) {
        wasm.wasmbrot_step(this.ptr, step_size);
    }
    /**
    */
    colorize() {
        wasm.wasmbrot_colorize(this.ptr);
    }
    /**
    * @returns {number}
    */
    depth() {
        var ret = wasm.wasmbrot_depth(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    colors() {
        var ret = wasm.wasmbrot_colors(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    left() {
        var ret = wasm.wasmbrot_left(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    right() {
        var ret = wasm.wasmbrot_right(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    top() {
        var ret = wasm.wasmbrot_top(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    down() {
        var ret = wasm.wasmbrot_down(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    pixel_size() {
        var ret = wasm.wasmbrot_pixel_size(this.ptr);
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

