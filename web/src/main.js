import * as wasm from './main_bg';

/**
* @returns {void}
*/
export function wasm_main() {
    return wasm.wasm_main();
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function __widl_f_get_element_by_id_Document(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_instanceof_HTMLCanvasElement(idx) { return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0; }

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

export function __widl_f_get_context_HTMLCanvasElement(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
}

export function __widl_instanceof_WebGL2RenderingContext(idx) { return getObject(idx) instanceof WebGL2RenderingContext ? 1 : 0; }

export function __widl_f_bind_vertex_array_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).bindVertexArray(getObject(arg1));
}

export function __widl_f_create_vertex_array_WebGL2RenderingContext(arg0) {

    const val = getObject(arg0).createVertexArray();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_delete_vertex_array_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).deleteVertexArray(getObject(arg1));
}

export function __widl_f_attach_shader_WebGL2RenderingContext(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
}

export function __widl_f_clear_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}

export function __widl_f_clear_color_WebGL2RenderingContext(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
}

export function __widl_f_compile_shader_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
}

export function __widl_f_create_buffer_WebGL2RenderingContext(arg0) {

    const val = getObject(arg0).createBuffer();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_create_program_WebGL2RenderingContext(arg0) {

    const val = getObject(arg0).createProgram();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_create_shader_WebGL2RenderingContext(arg0, arg1) {

    const val = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_delete_program_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).deleteProgram(getObject(arg1));
}

export function __widl_f_delete_shader_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).deleteShader(getObject(arg1));
}

export function __widl_f_detach_shader_WebGL2RenderingContext(arg0, arg1, arg2) {
    getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
}

export function __widl_f_draw_arrays_WebGL2RenderingContext(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {

        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let writeOffset = 0;
        while (true) {
            const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
            const { read, written } = cachedTextEncoder.encodeInto(arg, view);
            writeOffset += written;
            if (read === arg.length) {
                break;
            }
            arg = arg.substring(read);
            ptr = wasm.__wbindgen_realloc(ptr, size, size += arg.length * 3);
        }
        WASM_VECTOR_LEN = writeOffset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    };
}

export function __widl_f_get_program_info_log_WebGL2RenderingContext(ret, arg0, arg1) {
    const val = getObject(arg0).getProgramInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __widl_f_get_program_parameter_WebGL2RenderingContext(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0));
}

export function __widl_f_get_shader_info_log_WebGL2RenderingContext(ret, arg0, arg1) {
    const val = getObject(arg0).getShaderInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __widl_f_get_shader_parameter_WebGL2RenderingContext(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0));
}

export function __widl_f_link_program_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
}

export function __widl_f_shader_source_WebGL2RenderingContext(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    getObject(arg0).shaderSource(getObject(arg1), varg2);
}

export function __widl_f_use_program_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
}

export function __widl_f_attach_shader_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
}

export function __widl_f_clear_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}

export function __widl_f_clear_color_WebGLRenderingContext(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
}

export function __widl_f_compile_shader_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
}

export function __widl_f_create_buffer_WebGLRenderingContext(arg0) {

    const val = getObject(arg0).createBuffer();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_create_program_WebGLRenderingContext(arg0) {

    const val = getObject(arg0).createProgram();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_create_shader_WebGLRenderingContext(arg0, arg1) {

    const val = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __widl_f_delete_program_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).deleteProgram(getObject(arg1));
}

export function __widl_f_delete_shader_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).deleteShader(getObject(arg1));
}

export function __widl_f_detach_shader_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
}

export function __widl_f_draw_arrays_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
}

export function __widl_f_get_program_info_log_WebGLRenderingContext(ret, arg0, arg1) {
    const val = getObject(arg0).getProgramInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __widl_f_get_program_parameter_WebGLRenderingContext(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0));
}

export function __widl_f_get_shader_info_log_WebGLRenderingContext(ret, arg0, arg1) {
    const val = getObject(arg0).getShaderInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

export function __widl_f_get_shader_parameter_WebGLRenderingContext(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0));
}

export function __widl_f_link_program_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
}

export function __widl_f_shader_source_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    getObject(arg0).shaderSource(getObject(arg1), varg2);
}

export function __widl_f_use_program_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
}

export function __widl_instanceof_Window(idx) { return getObject(idx) instanceof Window ? 1 : 0; }

export function __widl_f_request_animation_frame_Window(arg0, arg1, exnptr) {
    try {
        return getObject(arg0).requestAnimationFrame(getObject(arg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}

export function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __wbg_newnoargs_9fab447a311888a5(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

export function __wbg_call_001e26aeb2fdef67(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

export function __wbindgen_boolean_get(i) {
    let v = getObject(i);
    return typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
}

export function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
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
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
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
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

export function __wbindgen_cb_drop(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

export function __wbindgen_closure_wrapper129(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(19);
    const d = wasm.__wbg_function_table.get(20);
    const cb = function() {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b);

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

export function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

wasm.__wbindgen_start();

