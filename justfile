alias b := build
alias u := uf2

# only for windows!!!!
set shell := ["powershell.exe", "-c"]

check:
	cargo check

# install the nesicary tools
setup:
	rustup target install thumbv6m-none-eabi
	cargo install elf2uf2-rs

clippy:
	cargo clippy

build:
	cargo build

build_release:
	cargo build --release

# build release and convert to uf2
uf2: build_release
	elf2uf2-rs target/thumbv6m-none-eabi/release/usb_logger_demo usb_logger_demo.uf2
	explorer.exe .

clean:
	cargo clean
	rm micro_demo.uf2