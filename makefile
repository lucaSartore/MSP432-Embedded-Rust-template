build:
	cargo build


flash:
	 arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/template --command=gdb_commands.txt

init:
	rustup default nightly
	rustup target add thumbv7em-none-eabihf
	git clone https://github.com/msp432-rust/msp432p401r-hal