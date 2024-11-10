cargo build --target wasm32-unknown-unknown
wasm-bindgen .\target\wasm32-unknown-unknown\debug\webgl_tuto.wasm --target web --out-dir .\generated\