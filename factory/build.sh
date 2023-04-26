#/bin/sh
mkdir -p res

# Web4 Profile Contract
echo "Compiling page contract to WASM";

yarn --cwd ../page build:contract

cp ../page/build/release/web4-profile.wasm ./res/contract.wasm

echo "Page compiled successfully";

# Factory Contract
echo "Compiling factory contract";

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

cp ./target/wasm32-unknown-unknown/release/factory.wasm ./res/factory.wasm

echo "Factory compiled successfully";


