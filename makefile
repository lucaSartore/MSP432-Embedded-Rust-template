build:
	cargo build

init:
	rustup default nightly
	rustup target add thumbv7em-none-eabihf
	git clone https://github.com/msp432-rust/msp432p401r-hal