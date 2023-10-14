default: build_and_flash

build_and_flash: build flash

build:
	cargo build

flash:
	gnome-terminal -- openocd
	sleep 0.5
	gnome-terminal -- arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/template --command=gdb_commands.txt

init:
	sudo apt install openocd
	sudo apt install gcc-arm-none-eabi
	sudo apt install gnome-terminal
	rustup default nightly
	rustup target add thumbv7em-none-eabihf
	git clone https://github.com/msp432-rust/msp432p401r-hal
	echo "you may need to manually install GNU ARM Embedded toolchain (see readme)"