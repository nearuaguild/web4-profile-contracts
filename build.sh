RUSTFLAGS='-C link-arg=-s' cargo build --manifest-path=page/Cargo.toml --target wasm32-unknown-unknown --release

cp ./page/target/wasm32-unknown-unknown/release/near_web4_profile.wasm ./res/contract.wasm