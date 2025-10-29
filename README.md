# ScarabOS
Scarab is a hobby OS, very minimal, created by me in order to learn systems programming concepts and make something interactive.
Scarab uses the x86_64-EFI architecture, If you're interested in trying this system I would highly recommend using a virtual machine such as qemu, most the project is designed around qemu


# Building

## Requirements
To build from source, make sure all requirements are installed
```sh
rustup
grub
nasm
qemu-full
xorriso
mtools
```
Most of these can be installed with any package manager, however for rust I recommend downloading that from [here](https://rustup.rs/)
## Setup rust
- Once installed we need to setup rust with a couple commands.
Firstly, we need to install the nightly version of rust from January 2024. Specifically this version so we can stablise the install here and not have the code break in newer nightly releases. This is enforced inside our .cargo folder
```sh
$ rustup install nightly
```
Then install Rust components
```sh
rustup component add rust-src
```

## Build virtual disk image
For the file system part of the OS, I'm currently using and testing with a virtual raw drive from qemu, generate it with this below:
```sh
qemu-img create -f raw disk.img 10M
```
it creates a 10Mb file for working with files, you can play with this number if you feel you must

## Compiling
The make file handles all the compilations right now, `clean` may be used to recompile all files at any time.
- **clean must be ran once before compiling, or it will fail**
```sh
make clean
```
This builds our .img file that we use to boot with qemu
```sh
make
```

# Running
simply
```
make run
```
Enjoy ~Top1055
