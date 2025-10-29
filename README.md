# 🪲 ScarabOS

A minimal hobby operating system written in Rust for x86_64-EFI architecture.

ScarabOS is a learning project focused on systems programming concepts. Purely a project for understanding how operating systems work from the ground up.

## 🚀 Quick Start

### Prerequisites

Ensure you have the following installed:

| Tool | Purpose |
|------|---------|
| `rustup` | Rust toolchain manager ([install here](https://rustup.rs/)) |
| `grub` | Bootloader |
| `nasm` | x86 assembler |
| `qemu-full` | Virtualization (recommended for testing) |
| `xorriso` | ISO image creation |
| `mtools` | FAT filesystem utilities |

**On Arch Linux:**
```bash
sudo pacman -S grub nasm qemu-full xorriso mtools
```

**On Ubuntu/Debian:**
```bash
sudo apt install grub-pc-bin nasm qemu-system-x86 xorriso mtools
```

---

## 🔧 Building from Source

### 1. Set up Rust

Install the nightly toolchain (pinned to January 2024 for stability):

```bash
rustup install nightly
rustup component add rust-src
```

### 2. Create virtual disk

Generate a raw disk image for the filesystem:

```bash
qemu-img create -f raw disk.img 10M
```

> **Note:** Adjust size as needed (e.g. `50M` for 50 megabytes)

### 3. Compile

**First time setup:**
```bash
make clean
```
> ⚠️ Run `make clean` once before your first build

**Build the OS:**
```bash
make
```

This creates `scarab.img`, your bootable OS image!

---

## 🎮 Running ScarabOS

Launch in QEMU with:

```bash
make run
```

You'll boot into a minimal terminal where you can explore the basic command system.
> Use alt + 2 to open up the Qemu terminal and then type q or quit to exit the VM

---

## 📚 Learning Resources

This project was heavily inspired by:
- [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann
- My Operating Systems professor in university
- The Pintos educational OS

---

## 🤝 Contributing

This is a personal learning project, but feel free to:
- **Fork** and experiment
- **Open issues** for interesting ideas
- **Share** your own OS development journey!
