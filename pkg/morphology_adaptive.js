/* @ts-self-types="./morphology_adaptive.d.ts" */

export class GameState {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameStateFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gamestate_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get_current_time() {
        const ret = wasm.gamestate_get_current_time(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_lap() {
        const ret = wasm.gamestate_get_lap(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_rotation_y() {
        const ret = wasm.gamestate_get_rotation_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_speed() {
        const ret = wasm.gamestate_get_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_x() {
        const ret = wasm.gamestate_get_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_y() {
        const ret = wasm.gamestate_get_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_z() {
        const ret = wasm.gamestate_get_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    is_completed() {
        const ret = wasm.gamestate_is_completed(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {string} args_json
     * @param {string} weights_json
     */
    load_policy(args_json, weights_json) {
        const ptr0 = passStringToWasm0(args_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(weights_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.gamestate_load_policy(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
     * @param {number} total_laps
     * @param {number} morphology
     */
    constructor(total_laps, morphology) {
        const ret = wasm.gamestate_new_wasm(total_laps, morphology);
        this.__wbg_ptr = ret >>> 0;
        GameStateFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} now
     */
    start_race(now) {
        wasm.gamestate_start_race(this.__wbg_ptr, now);
    }
    /**
     * @param {number} delta
     * @param {number} now
     * @param {string[]} keys
     */
    update(delta, now, keys) {
        const ptr0 = passArrayJsValueToWasm0(keys, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.gamestate_update(this.__wbg_ptr, delta, now, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get current_time() {
        const ret = wasm.__wbg_get_gamestate_current_time(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get lap() {
        const ret = wasm.__wbg_get_gamestate_lap(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get last_angle() {
        const ret = wasm.__wbg_get_gamestate_last_angle(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get race_completed() {
        const ret = wasm.__wbg_get_gamestate_race_completed(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get race_started() {
        const ret = wasm.__wbg_get_gamestate_race_started(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    get rotation_y() {
        const ret = wasm.__wbg_get_gamestate_rotation_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get speed() {
        const ret = wasm.__wbg_get_gamestate_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get start_time() {
        const ret = wasm.__wbg_get_gamestate_start_time(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get total_angle() {
        const ret = wasm.__wbg_get_gamestate_total_angle(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get total_laps() {
        const ret = wasm.__wbg_get_gamestate_total_laps(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_gamestate_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_gamestate_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_gamestate_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set current_time(arg0) {
        wasm.__wbg_set_gamestate_current_time(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set lap(arg0) {
        wasm.__wbg_set_gamestate_lap(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set last_angle(arg0) {
        wasm.__wbg_set_gamestate_last_angle(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} arg0
     */
    set race_completed(arg0) {
        wasm.__wbg_set_gamestate_race_completed(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} arg0
     */
    set race_started(arg0) {
        wasm.__wbg_set_gamestate_race_started(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set rotation_y(arg0) {
        wasm.__wbg_set_gamestate_rotation_y(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set speed(arg0) {
        wasm.__wbg_set_gamestate_speed(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set start_time(arg0) {
        wasm.__wbg_set_gamestate_start_time(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set total_angle(arg0) {
        wasm.__wbg_set_gamestate_total_angle(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set total_laps(arg0) {
        wasm.__wbg_set_gamestate_total_laps(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_gamestate_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_gamestate_y(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_gamestate_z(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) GameState.prototype[Symbol.dispose] = GameState.prototype.free;

/**
 * @enum {0 | 1 | 2}
 */
export const Morphology = Object.freeze({
    Biped: 0, "0": "Biped",
    Quadruped: 1, "1": "Quadruped",
    Hexapod: 2, "2": "Hexapod",
});

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_string_get_3e5751597f39a112: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./morphology_adaptive_bg.js": import0,
    };
}

const GameStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gamestate_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('morphology_adaptive_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
