.PHONY: clean

clean:
	cd backend && cargo clean
	cd frontend && cargo clean

build_releases:
	cd backend && cargo build --release
	cd frontend && cargo build --release
build_backend:
	cd backend && cargo build --release
build_frontend:
	cd frontend && cargo build --release