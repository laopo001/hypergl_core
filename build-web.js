const { execSync } = require('child_process');

let file = 'main';

execSync(`cargo build --example ${file} --target wasm32-unknown-unknown`)
execSync(`wasm-bindgen target/wasm32-unknown-unknown/debug/examples/${file}.wasm --out-dir web/src/`)
console.log('open 127.0.0.1:8100')
execSync(`npm run start`, { cwd: 'web' })
