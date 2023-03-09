#!/bin/sh

# Check for out folder
DIR="./out/isodir/boot/grub"
if [ -d "$DIR" ]; then
	echo "Output folder exists."
else
	echo "Creating output folder..."
	mkdir mkdir -p out/isodir/boot/grub/
fi

# Check for rust
if rustup -V; then
	echo "Rustup installed already."
else
	# Install rust
	echo "Rustup missing, please follow the installation process to continue."
	curl https://sh.rustup.rs -sSf | sh
fi

rustup install nightly
cargo install xargo
rustup component add rust-src
apt install nasm qemu xorriso qemu-system
make
make run
