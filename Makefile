./out/scarab.img: ./out/isodir/boot/kernel.bin
	cp ./boot/grub.cfg ./out/isodir/boot/grub/grub.cfg
	grub-mkrescue -o ./out/bootable.img ./out/isodir

./out/isodir/boot/grub/grub.cfg: ./boot/grub.cfg
	cp ./boot/grub.cfg ./out/isodir/boot/grub/grub.cfg

./out/isodir/boot/kernel.bin: ./out/boot.o ./out/long_mode_init.o ./out/multiboot_header.o ./out/libscarab.a ./boot/linker.ld
	ld -n --gc-sections -o ./out/isodir/boot/kernel.bin -T ./boot/linker.ld ./out/multiboot_header.o ./out/boot.o ./out/long_mode_init.o ./target/x86_64-scarab_os/debug/libscarab.a

# Rust code
./out/libscarab.a: ./src/lib.rs
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-scarab_os

# Assembly booter

./out/multiboot_header.o: ./boot/multiboot_header.asm
	nasm -felf64 ./boot/multiboot_header.asm -o ./out/multiboot_header.o

./out/long_mode_init.o: ./boot/long_mode_init.asm
	nasm -felf64 ./boot/long_mode_init.asm -o ./out/long_mode_init.o

./out/boot.o: ./boot/boot.asm
	nasm -felf64 ./boot/boot.asm -o ./out/boot.o

./out:
	mkdir out

run:
	qemu-system-x86_64 -display curses -drive format=raw,file=out/bootable.img

clean:
	rm -rf ./out/*
	mkdir -p ./out/isodir/boot/grub
	rm -rf target
	@RUST_TARGET_PATH=$(shell pwd) xargo clean
	xargo clean
	cargo clean
