#/bin/sh
yarn --cwd ./page build:contract

cp ./page/build/release/web4-profile.wasm ./res/contract.wasm