#!/usr/bin/env zx

if (argv.example) {
    await $`cargo build --target wasm32-unknown-unknown --example ${argv.example}`
    await $`wasm-bindgen ./target/wasm32-unknown-unknown/debug/examples/${argv.example}.wasm --out-dir pkg --web --out-name webgpu_learn`
} else {
    await $`cargo build --target wasm32-unknown-unknown`
    await $`wasm-bindgen ./target/wasm32-unknown-unknown/debug/webgpu_learn.wasm --out-dir pkg --web`
}

