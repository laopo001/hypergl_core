#!/bin/bash 
# set -x  set -e

fileName=$1
if [$fileName == ''] 
then
    fileName="main2"
fi

COLOR_REST="$(tput sgr0)"
COLOR_GREEN="$(tput setaf 2)"

for loop in 1
do
    echo "${COLOR_GREEN}cargo build --example ${fileName} --target wasm32-unknown-unknown${COLOR_REST}"
    cargo build --example ${fileName} --target wasm32-unknown-unknown || break 
    echo "${COLOR_GREEN}wasm-bindgen target/wasm32-unknown-unknown/debug/examples/${fileName}.wasm --out-dir web/src/${COLOR_REST}" 
    wasm-bindgen target/wasm32-unknown-unknown/debug/examples/${fileName}.wasm --out-dir web/src/  || break 

    echo "import(\"./${fileName}\")
    .catch(e => console.error(\"Error importing index.js:\", e));" > web/src/bootstrap.js  

    (cd web && npm run start) 
done


 
