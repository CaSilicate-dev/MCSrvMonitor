build:
	cargo build --release
	docker build -t mcsrvmon-be .

.PHONY: build