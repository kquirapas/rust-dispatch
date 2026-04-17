.PHONY: run-static run-dynamic

all: build

build:
	cargo build --profile no-opts

run-static:
	cargo run --profile no-opts
run-dynamic:
	cargo run --profile no-opts
