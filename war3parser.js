let wasm;

let cachedUint8ClampedArrayMemory0 = null;

function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

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
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
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

let cachedInt32ArrayMemory0 = null;

function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const WasmForceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmforce_free(ptr >>> 0, 1));

export class WasmForce {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmForce.prototype);
        obj.__wbg_ptr = ptr;
        WasmForceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmForce)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmForceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmforce_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get flags() {
        const ret = wasm.__wbg_get_wasmforce_flags(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set flags(arg0) {
        wasm.__wbg_set_wasmforce_flags(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get player_masks() {
        const ret = wasm.__wbg_get_wasmforce_player_masks(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set player_masks(arg0) {
        wasm.__wbg_set_wasmforce_player_masks(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmforce_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmforce_name(this.__wbg_ptr, ptr0, len0);
    }
}

const WasmImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmimage_free(ptr >>> 0, 1));

export class WasmImage {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmImage.prototype);
        obj.__wbg_ptr = ptr;
        WasmImageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmImage)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmImageFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmimage_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get width() {
        const ret = wasm.__wbg_get_wasmimage_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set width(arg0) {
        wasm.__wbg_set_wasmimage_width(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get height() {
        const ret = wasm.__wbg_get_wasmimage_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set height(arg0) {
        wasm.__wbg_set_wasmimage_height(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {ImageData}
     */
    get data() {
        const ret = wasm.__wbg_get_wasmimage_data(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {ImageData} arg0
     */
    set data(arg0) {
        wasm.__wbg_set_wasmimage_data(this.__wbg_ptr, arg0);
    }
}

const WasmMapInfoFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmapinfo_free(ptr >>> 0, 1));

export class WasmMapInfo {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmMapInfo.prototype);
        obj.__wbg_ptr = ptr;
        WasmMapInfoFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMapInfoFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmapinfo_free(ptr, 0);
    }
    /**
     * @returns {WasmW3i | undefined}
     */
    get map_info() {
        const ret = wasm.__wbg_get_wasmmapinfo_map_info(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmW3i.__wrap(ret);
    }
    /**
     * @param {WasmW3i | null} [arg0]
     */
    set map_info(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, WasmW3i);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_wasmmapinfo_map_info(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {WasmImage[]}
     */
    get images() {
        const ret = wasm.__wbg_get_wasmmapinfo_images(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmImage[]} arg0
     */
    set images(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmmapinfo_images(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {Uint8Array} buffer
     * @returns {WasmMapInfo | undefined}
     */
    static new(buffer) {
        const ret = wasm.wasmmapinfo_new(buffer);
        return ret === 0 ? undefined : WasmMapInfo.__wrap(ret);
    }
}

const WasmPlayerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmplayer_free(ptr >>> 0, 1));

export class WasmPlayer {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPlayer.prototype);
        obj.__wbg_ptr = ptr;
        WasmPlayerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmPlayer)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPlayerFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmplayer_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get id() {
        const ret = wasm.__wbg_get_wasmplayer_id(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_wasmplayer_id(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get player_type() {
        const ret = wasm.__wbg_get_wasmplayer_player_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set player_type(arg0) {
        wasm.__wbg_set_wasmplayer_player_type(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get race() {
        const ret = wasm.__wbg_get_wasmplayer_race(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set race(arg0) {
        wasm.__wbg_set_wasmplayer_race(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get is_fixed_start_position() {
        const ret = wasm.__wbg_get_wasmplayer_is_fixed_start_position(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set is_fixed_start_position(arg0) {
        wasm.__wbg_set_wasmplayer_is_fixed_start_position(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmplayer_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmplayer_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Float32Array}
     */
    get start_location() {
        const ret = wasm.__wbg_get_wasmplayer_start_location(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Float32Array} arg0
     */
    set start_location(arg0) {
        wasm.__wbg_set_wasmplayer_start_location(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get ally_low_priorities() {
        const ret = wasm.__wbg_get_wasmplayer_ally_low_priorities(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set ally_low_priorities(arg0) {
        wasm.__wbg_set_wasmplayer_ally_low_priorities(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get ally_high_priorities() {
        const ret = wasm.__wbg_get_wasmplayer_ally_high_priorities(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set ally_high_priorities(arg0) {
        wasm.__wbg_set_wasmplayer_ally_high_priorities(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get known1() {
        const ret = wasm.__wbg_get_wasmplayer_known1(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array | null} [arg0]
     */
    set known1(arg0) {
        wasm.__wbg_set_wasmplayer_known1(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
}

const WasmRandomItemFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmrandomitem_free(ptr >>> 0, 1));

export class WasmRandomItem {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmRandomItem.prototype);
        obj.__wbg_ptr = ptr;
        WasmRandomItemFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmRandomItem)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmRandomItemFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmrandomitem_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get chance() {
        const ret = wasm.__wbg_get_wasmrandomitem_chance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set chance(arg0) {
        wasm.__wbg_set_wasmrandomitem_chance(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array}
     */
    get id() {
        const ret = wasm.__wbg_get_wasmrandomitem_id(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_wasmrandomitem_id(this.__wbg_ptr, arg0);
    }
}

const WasmRandomItemSetFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmrandomitemset_free(ptr >>> 0, 1));

export class WasmRandomItemSet {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmRandomItemSet.prototype);
        obj.__wbg_ptr = ptr;
        WasmRandomItemSetFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmRandomItemSet)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmRandomItemSetFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmrandomitemset_free(ptr, 0);
    }
    /**
     * @returns {WasmRandomItem[]}
     */
    get items() {
        const ret = wasm.__wbg_get_wasmrandomitemset_items(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmRandomItem[]} arg0
     */
    set items(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmrandomitemset_items(this.__wbg_ptr, ptr0, len0);
    }
}

const WasmRandomItemTableFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmrandomitemtable_free(ptr >>> 0, 1));

export class WasmRandomItemTable {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmRandomItemTable.prototype);
        obj.__wbg_ptr = ptr;
        WasmRandomItemTableFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmRandomItemTable)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmRandomItemTableFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmrandomitemtable_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get id() {
        const ret = wasm.__wbg_get_wasmplayer_player_type(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_wasmplayer_player_type(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmrandomitemtable_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmforce_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmRandomItemSet[]}
     */
    get sets() {
        const ret = wasm.__wbg_get_wasmrandomitemtable_sets(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmRandomItemSet[]} arg0
     */
    set sets(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmrandomitemtable_sets(this.__wbg_ptr, ptr0, len0);
    }
}

const WasmRandomUnitFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmrandomunit_free(ptr >>> 0, 1));

export class WasmRandomUnit {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmRandomUnit.prototype);
        obj.__wbg_ptr = ptr;
        WasmRandomUnitFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmRandomUnit)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmRandomUnitFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmrandomunit_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get chance() {
        const ret = wasm.__wbg_get_wasmforce_flags(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set chance(arg0) {
        wasm.__wbg_set_wasmforce_flags(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array[]}
     */
    get ids() {
        const ret = wasm.__wbg_get_wasmrandomunit_ids(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Uint8Array[]} arg0
     */
    set ids(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmrandomunit_ids(this.__wbg_ptr, ptr0, len0);
    }
}

const WasmRandomUnitTableFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmrandomunittable_free(ptr >>> 0, 1));

export class WasmRandomUnitTable {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmRandomUnitTable.prototype);
        obj.__wbg_ptr = ptr;
        WasmRandomUnitTableFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmRandomUnitTable)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmRandomUnitTableFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmrandomunittable_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get id() {
        const ret = wasm.__wbg_get_wasmrandomunittable_id(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_wasmrandomunittable_id(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmrandomunittable_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmforce_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get position() {
        const ret = wasm.__wbg_get_wasmplayer_ally_low_priorities(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set position(arg0) {
        wasm.__wbg_set_wasmplayer_ally_low_priorities(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Int32Array}
     */
    get column_types() {
        const ret = wasm.__wbg_get_wasmrandomunittable_column_types(this.__wbg_ptr);
        var v1 = getArrayI32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Int32Array} arg0
     */
    set column_types(arg0) {
        const ptr0 = passArray32ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmrandomunittable_column_types(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmRandomUnit[]}
     */
    get units() {
        const ret = wasm.__wbg_get_wasmrandomunittable_units(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmRandomUnit[]} arg0
     */
    set units(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmrandomunittable_units(this.__wbg_ptr, ptr0, len0);
    }
}

const WasmTechAvailabilityChangeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtechavailabilitychange_free(ptr >>> 0, 1));

export class WasmTechAvailabilityChange {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmTechAvailabilityChange.prototype);
        obj.__wbg_ptr = ptr;
        WasmTechAvailabilityChangeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmTechAvailabilityChange)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTechAvailabilityChangeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtechavailabilitychange_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get player_flags() {
        const ret = wasm.__wbg_get_wasmrandomitem_chance(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set player_flags(arg0) {
        wasm.__wbg_set_wasmrandomitem_chance(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array}
     */
    get id() {
        const ret = wasm.__wbg_get_wasmtechavailabilitychange_id(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_wasmtechavailabilitychange_id(this.__wbg_ptr, arg0);
    }
}

const WasmUpgradeAvailabilityChangeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmupgradeavailabilitychange_free(ptr >>> 0, 1));

export class WasmUpgradeAvailabilityChange {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmUpgradeAvailabilityChange.prototype);
        obj.__wbg_ptr = ptr;
        WasmUpgradeAvailabilityChangeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmUpgradeAvailabilityChange)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmUpgradeAvailabilityChangeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmupgradeavailabilitychange_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get player_flags() {
        const ret = wasm.__wbg_get_wasmrandomitem_chance(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set player_flags(arg0) {
        wasm.__wbg_set_wasmrandomitem_chance(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array}
     */
    get id() {
        const ret = wasm.__wbg_get_wasmupgradeavailabilitychange_id(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_wasmupgradeavailabilitychange_id(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get level_affected() {
        const ret = wasm.__wbg_get_wasmupgradeavailabilitychange_level_affected(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set level_affected(arg0) {
        wasm.__wbg_set_wasmupgradeavailabilitychange_level_affected(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get availability() {
        const ret = wasm.__wbg_get_wasmforce_flags(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set availability(arg0) {
        wasm.__wbg_set_wasmforce_flags(this.__wbg_ptr, arg0);
    }
}

const WasmW3iFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmw3i_free(ptr >>> 0, 1));

export class WasmW3i {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmW3i.prototype);
        obj.__wbg_ptr = ptr;
        WasmW3iFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmW3iFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmw3i_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get version() {
        const ret = wasm.__wbg_get_wasmw3i_version(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set version(arg0) {
        wasm.__wbg_set_wasmw3i_version(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get saves() {
        const ret = wasm.__wbg_get_wasmw3i_saves(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set saves(arg0) {
        wasm.__wbg_set_wasmw3i_saves(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get editor_version() {
        const ret = wasm.__wbg_get_wasmw3i_editor_version(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set editor_version(arg0) {
        wasm.__wbg_set_wasmw3i_editor_version(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint32Array | undefined}
     */
    get build_version() {
        const ret = wasm.__wbg_get_wasmw3i_build_version(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint32Array | null} [arg0]
     */
    set build_version(arg0) {
        wasm.__wbg_set_wasmplayer_known1(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get author() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_author(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set author(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_author(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get description() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_description(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set description(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_description(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get recommended_players() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_recommended_players(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set recommended_players(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_recommended_players(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {Float32Array}
     */
    get camera_bounds() {
        const ret = wasm.__wbg_get_wasmw3i_camera_bounds(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Float32Array} arg0
     */
    set camera_bounds(arg0) {
        wasm.__wbg_set_wasmw3i_camera_bounds(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint32Array}
     */
    get camera_bounds_complements() {
        const ret = wasm.__wbg_get_wasmw3i_camera_bounds_complements(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint32Array} arg0
     */
    set camera_bounds_complements(arg0) {
        wasm.__wbg_set_wasmw3i_camera_bounds_complements(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint32Array}
     */
    get playable_size() {
        const ret = wasm.__wbg_get_wasmw3i_playable_size(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint32Array} arg0
     */
    set playable_size(arg0) {
        wasm.__wbg_set_wasmw3i_playable_size(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get flags() {
        const ret = wasm.__wbg_get_wasmw3i_flags(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set flags(arg0) {
        wasm.__wbg_set_wasmw3i_flags(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get tileset() {
        const ret = wasm.__wbg_get_wasmw3i_tileset(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set tileset(arg0) {
        wasm.__wbg_set_wasmw3i_tileset(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get campaign_background() {
        const ret = wasm.__wbg_get_wasmw3i_campaign_background(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set campaign_background(arg0) {
        wasm.__wbg_set_wasmw3i_campaign_background(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string | undefined}
     */
    get loading_screen_model() {
        const ret = wasm.__wbg_get_wasmw3i_loading_screen_model(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set loading_screen_model(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_loading_screen_model(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get loading_screen_text() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_loading_screen_text(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set loading_screen_text(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_loading_screen_text(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get loading_screen_title() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_loading_screen_title(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set loading_screen_title(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_loading_screen_title(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get loading_screen_subtitle() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_loading_screen_subtitle(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set loading_screen_subtitle(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_loading_screen_subtitle(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get loading_screen() {
        const ret = wasm.__wbg_get_wasmw3i_loading_screen(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set loading_screen(arg0) {
        wasm.__wbg_set_wasmw3i_loading_screen(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string | undefined}
     */
    get prologue_screen_model() {
        const ret = wasm.__wbg_get_wasmw3i_prologue_screen_model(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set prologue_screen_model(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_prologue_screen_model(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get prologue_screen_text() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_prologue_screen_text(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set prologue_screen_text(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_prologue_screen_text(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get prologue_screen_title() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_prologue_screen_title(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set prologue_screen_title(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_prologue_screen_title(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get prologue_screen_subtitle() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_wasmw3i_prologue_screen_subtitle(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set prologue_screen_subtitle(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_prologue_screen_subtitle(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number | undefined}
     */
    get use_terrain_fog() {
        const ret = wasm.__wbg_get_wasmw3i_use_terrain_fog(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set use_terrain_fog(arg0) {
        wasm.__wbg_set_wasmw3i_use_terrain_fog(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {Float32Array | undefined}
     */
    get fog_height() {
        const ret = wasm.__wbg_get_wasmw3i_fog_height(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Float32Array | null} [arg0]
     */
    set fog_height(arg0) {
        wasm.__wbg_set_wasmw3i_fog_height(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {number | undefined}
     */
    get fog_density() {
        const ret = wasm.__wbg_get_wasmw3i_fog_density(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set fog_density(arg0) {
        wasm.__wbg_set_wasmw3i_fog_density(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get fog_color() {
        const ret = wasm.__wbg_get_wasmw3i_fog_color(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array | null} [arg0]
     */
    set fog_color(arg0) {
        wasm.__wbg_set_wasmw3i_fog_color(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {number | undefined}
     */
    get global_weather() {
        const ret = wasm.__wbg_get_wasmw3i_global_weather(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set global_weather(arg0) {
        wasm.__wbg_set_wasmw3i_global_weather(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {string | undefined}
     */
    get sound_environment() {
        const ret = wasm.__wbg_get_wasmw3i_sound_environment(this.__wbg_ptr);
        let v1;
        if (ret[0] !== 0) {
            v1 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v1;
    }
    /**
     * @param {string | null} [arg0]
     */
    set sound_environment(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_sound_environment(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number | undefined}
     */
    get light_environment_tileset() {
        const ret = wasm.__wbg_get_wasmw3i_light_environment_tileset(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set light_environment_tileset(arg0) {
        wasm.__wbg_set_wasmw3i_light_environment_tileset(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0);
    }
    /**
     * @returns {Uint8Array | undefined}
     */
    get water_vertex_color() {
        const ret = wasm.__wbg_get_wasmw3i_water_vertex_color(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array | null} [arg0]
     */
    set water_vertex_color(arg0) {
        wasm.__wbg_set_wasmw3i_water_vertex_color(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
    }
    /**
     * @returns {number | undefined}
     */
    get script_mode() {
        const ret = wasm.__wbg_get_wasmw3i_script_mode(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set script_mode(arg0) {
        wasm.__wbg_set_wasmw3i_script_mode(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {number | undefined}
     */
    get graphics_mode() {
        const ret = wasm.__wbg_get_wasmw3i_graphics_mode(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set graphics_mode(arg0) {
        wasm.__wbg_set_wasmw3i_graphics_mode(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {number | undefined}
     */
    get unknown1() {
        const ret = wasm.__wbg_get_wasmw3i_unknown1(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number | null} [arg0]
     */
    set unknown1(arg0) {
        wasm.__wbg_set_wasmw3i_unknown1(this.__wbg_ptr, isLikeNone(arg0) ? 0x100000001 : (arg0) >>> 0);
    }
    /**
     * @returns {WasmPlayer[]}
     */
    get players() {
        const ret = wasm.__wbg_get_wasmw3i_players(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmPlayer[]} arg0
     */
    set players(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_players(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmForce[]}
     */
    get forces() {
        const ret = wasm.__wbg_get_wasmw3i_forces(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmForce[]} arg0
     */
    set forces(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_forces(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmUpgradeAvailabilityChange[]}
     */
    get upgrade_availability_changes() {
        const ret = wasm.__wbg_get_wasmw3i_upgrade_availability_changes(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmUpgradeAvailabilityChange[]} arg0
     */
    set upgrade_availability_changes(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_upgrade_availability_changes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmTechAvailabilityChange[]}
     */
    get tech_availability_changes() {
        const ret = wasm.__wbg_get_wasmw3i_tech_availability_changes(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmTechAvailabilityChange[]} arg0
     */
    set tech_availability_changes(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_tech_availability_changes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmRandomUnitTable[]}
     */
    get random_unit_tables() {
        const ret = wasm.__wbg_get_wasmw3i_random_unit_tables(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmRandomUnitTable[]} arg0
     */
    set random_unit_tables(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_random_unit_tables(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {WasmRandomItemTable[]}
     */
    get random_item_tables() {
        const ret = wasm.__wbg_get_wasmw3i_random_item_tables(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {WasmRandomItemTable[]} arg0
     */
    set random_item_tables(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_wasmw3i_random_item_tables(this.__wbg_ptr, ptr0, len0);
    }
}

const WasmWtsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmwts_free(ptr >>> 0, 1));

export class WasmWts {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmWtsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmwts_free(ptr, 0);
    }
    /**
     * @returns {Map<any, any>}
     */
    get string_map() {
        const ret = wasm.__wbg_get_wasmwts_string_map(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Map<any, any>} arg0
     */
    set string_map(arg0) {
        wasm.__wbg_set_wasmwts_string_map(this.__wbg_ptr, arg0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_length_3b4f022188ae8db6 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_length_6ca527665d89694d = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_length_a446193dc22c12f8 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_e6b7e69acd4c7354 = function(arg0, arg1, arg2) {
        const ret = new Float32Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_f1dead44d1fc7212 = function(arg0, arg1, arg2) {
        const ret = new Uint32Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithlength_5a5efe313cfd59f1 = function(arg0) {
        const ret = new Float32Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithlength_a381634e90c276d4 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithlength_bd3de93688d68fbc = function(arg0) {
        const ret = new Uint32Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithu8clampedarrayandsh_7ea6ee082a25bc85 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_10bad9bee0e9c58b = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_set_d23661d19148b229 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_wasmforce_new = function(arg0) {
        const ret = WasmForce.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmforce_unwrap = function(arg0) {
        const ret = WasmForce.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmimage_new = function(arg0) {
        const ret = WasmImage.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmimage_unwrap = function(arg0) {
        const ret = WasmImage.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmplayer_new = function(arg0) {
        const ret = WasmPlayer.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmplayer_unwrap = function(arg0) {
        const ret = WasmPlayer.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomitem_new = function(arg0) {
        const ret = WasmRandomItem.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomitem_unwrap = function(arg0) {
        const ret = WasmRandomItem.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomitemset_new = function(arg0) {
        const ret = WasmRandomItemSet.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomitemset_unwrap = function(arg0) {
        const ret = WasmRandomItemSet.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomitemtable_new = function(arg0) {
        const ret = WasmRandomItemTable.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomitemtable_unwrap = function(arg0) {
        const ret = WasmRandomItemTable.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomunit_new = function(arg0) {
        const ret = WasmRandomUnit.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomunit_unwrap = function(arg0) {
        const ret = WasmRandomUnit.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomunittable_new = function(arg0) {
        const ret = WasmRandomUnitTable.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmrandomunittable_unwrap = function(arg0) {
        const ret = WasmRandomUnitTable.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmtechavailabilitychange_new = function(arg0) {
        const ret = WasmTechAvailabilityChange.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmtechavailabilitychange_unwrap = function(arg0) {
        const ret = WasmTechAvailabilityChange.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmupgradeavailabilitychange_new = function(arg0) {
        const ret = WasmUpgradeAvailabilityChange.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_wasmupgradeavailabilitychange_unwrap = function(arg0) {
        const ret = WasmUpgradeAvailabilityChange.__unwrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedInt32ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    cachedUint8ClampedArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('war3parser_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
