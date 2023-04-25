RUSTFLAGS='-C link-arg=-s' cargo build --manifest-path=factory/Cargo.toml --target wasm32-unknown-unknown --release

mkdir -p res

cp ./factory/target/wasm32-unknown-unknown/release/factory.wasm ./res/factory.wasm