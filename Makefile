.PHONY: run dump

all: build

build:
	cargo build --release
run:
	cargo run --release
dump:
	objdump -d --demangle --disassemble=dp::main target/release/dp > dump/dump

dump-all:
	objdump -d --demangle target/release/dp > dump/full/dump
