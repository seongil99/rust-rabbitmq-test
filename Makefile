
all: test build

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all -- -D warnings

build: fmt clippy
	cargo build

test: build
	cargo test --all -- --nocapture

watch: build
	cargo watch -x 'test --all -- --nocapture'


run-benchmark:
	cargo run --release -p benchmark

rabbitmq-server:
	docker run -it --rm --name rabbitmq \
		-p 5672:5672 -p 15672:15672 \
		rabbitmq:3.11-management

help:
	cat Makefile
