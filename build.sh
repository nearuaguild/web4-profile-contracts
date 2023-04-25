#/bin/sh
yarn --cwd ./page build:contract

mkdir -p res

cp ./page/build/release/web4-profile.wasm ./res/contract.wasm