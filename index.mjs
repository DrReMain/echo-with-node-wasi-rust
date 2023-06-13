import {resolve,dirname} from 'node:path'
import {fileURLToPath} from 'node:url'
import {readFile} from 'node:fs/promises'
import {argv, env} from 'node:process'

import {WASI} from 'wasi'

const wasi = new WASI({
    args: argv.slice(1),
    env,
    preopens: {
        '/sandbox': resolve(dirname(fileURLToPath(import.meta.url)), "sandbox")
    }
});

const importObject = {wasi_snapshot_preview1: wasi.wasiImport};

const wasmURL = new URL('./crates/echor/target/wasm32-wasi/release/echor.wasm', import.meta.url);

const wasm = await WebAssembly.compile(await readFile(wasmURL));

const instance = await WebAssembly.instantiate(wasm, importObject);

wasi.start(instance);
