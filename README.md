# ScarabOS
A very minimal OS

# Requirements
`rustup`
`grub`
`nasm`
`qemu`
`xorriso`
`qemu-system`
if running via debian or ubuntu the `./install.sh` file should manage these for other systems, the requirements will need to manually be installed
```
$ ./install.sh
```

# Building

to build from source
```
$ make
```
clean may be used to recompile all files
```
$ make clean
```

# Running
simply
```
$ make run
```
aka run an existing .img file
```
$ qemu-system-x86_64 -drive format=raw,file=[IMAGE_NAME].img
```
add curses display if running in terminal
```
$ qemu-system-x86_64 -display curses -drive format=raw,file=[IMAGE_NAME].img
```
