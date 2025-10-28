./out/scarab.img: ./out/isodir/boot/kernel.bin
	cp ./boot/grub.cfg ./out/isodir/boot/grub/grub.cfg
	grub-mkrescue -o ./out/scarab.img ./out/isodir

./out/isodir/boot/grub/grub.cfg: ./boot/grub.cfg
	cp ./boot/grub.cfg ./out/isodir/boot/grub/grub.cfg

./out/isodir/boot/kernel.bin: ./out/boot.o ./out/rust_handshake.o ./out/multiboot_header.o ./out/libscarab.a ./boot/linker.ld
	ld -n --gc-sections -o ./out/isodir/boot/kernel.bin -T ./boot/linker.ld ./out/multiboot_header.o ./out/boot.o ./out/rust_handshake.o ./target/x86_64-scarab_os/release/libscarab.a

# Rust code
./out/libscarab.a: ./src/lib.rs
	@RUST_TARGET_PATH=$(shell pwd) cargo build --release

# Assembly booter

./out/multiboot_header.o: ./boot/multiboot_header.asm
	nasm -felf64 ./boot/multiboot_header.asm -o ./out/multiboot_header.o

./out/rust_handshake.o: ./boot/rust_handshake.asm
	nasm -felf64 ./boot/rust_handshake.asm -o ./out/rust_handshake.o

./out/boot.o: ./boot/boot.asm
	nasm -felf64 ./boot/boot.asm -o ./out/boot.o

./out:
	mkdir out

run:
	qemu-system-x86_64 -display curses -drive format=raw,file=out/scarab.img

clean:
	rm -rf ./out/*
	mkdir -p ./out/isodir/boot/grub
	rm -rf target
	cargo clean
