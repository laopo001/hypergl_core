const { execSync } = require('child_process');
const fs = require('fs');

let fileName = 'main';

execSync(`cargo build --example ${fileName} --target wasm32-unknown-unknown`)
execSync(`wasm-bindgen target/wasm32-unknown-unknown/debug/examples/${fileName}.wasm --out-dir web/src/`)
console.log('open 127.0.0.1:8100')
let text = `
import("./${fileName}")
  .catch(e => console.error("Error importing index.js:", e));
`
fs.writeFileSync('web/src/bootstrap.js', text);
execSync(`npm run start`, { cwd: 'web' })
