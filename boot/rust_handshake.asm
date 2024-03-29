global rust_handshake
extern rust_main

section .text
bits 64
rust_handshake:

    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Offload to rust
    extern rust_main
    call rust_main

    hlt
