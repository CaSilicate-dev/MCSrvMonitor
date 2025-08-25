all: build_frontend build_backend

build_frontend:
	cd frontend && npm install && npm run build

build_backend: frontend
	cargo build --release

clean:
	cargo clean
	cd frontend && rm -rf build node_modules

run:
	cargo run

start:
	cd frontend && npm install && npm run build
	cargo build --release
	cargo run --release