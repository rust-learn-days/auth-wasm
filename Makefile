build:
	rustup target add wasm32-wasi
	cargo build --target wasm32-wasi --release
	cp target/wasm32-wasi/release/auth_wasm.wasm ./plugin.wasm

push:
	sealos build -t sealos.hub:5000/oci/wasm-auth:v2  -f Dockerfile   .
	sealos push sealos.hub:5000/oci/wasm-auth:v2
