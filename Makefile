build:
	rustup target add wasm32-wasi
	cargo build --target wasm32-wasi --release
	cp target/wasm32-wasi/release/auth_wasm.wasm ./plugin.wasm

VERSION?=v5

push:
	sealos build -t sealos.hub:5000/oci/wasm-auth:$(VERSION)  -f Dockerfile   .
	sealos push sealos.hub:5000/oci/wasm-auth:$(VERSION)
	sed -i 's#sealos.hub:5000/oci/wasm-auth:v5#sealos.hub:5000/oci/wasm-auth:$(VERSION)#g'  deploy/charts/registry-proxy/templates/wasm.yaml
