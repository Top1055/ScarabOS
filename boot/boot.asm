global start
extern rust_handshake

section .text
bits 32
start:
    ; Grab stack pointer from GRUB
    mov esp, stack_top

    ; Setup paging for long mode
    call set_up_page_tables
    call enable_paging

    ; load the 64-bit GDT
    lgdt [gdt64.pointer]

    jmp gdt64.code:rust_handshake

    ; move 'ok' characters to VGA buffer
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

set_up_page_tables:
    ; map first page-map table to page pointer table
    mov eax, page_pointer_table
    or eax, 0b11
    mov [page_map_table], eax

    ; map first page pointer table entry to page table table
    mov eax, page_table
    or eax, 0b11
    mov [page_pointer_table], eax

    ; map each page table entry to a 2M page
    mov ecx, 0

.map_page_table:
    ; map ecx-th page_table entry to a page that starts at address 2MiB*ecx
    mov eax, 0x200000  ; 2MiB
    mul ecx            ; start address of ecx-th page
    or eax, 0b10000011 ; present + writable + huge
    mov [page_table + ecx * 8], eax ; map entry

    ; This is a for loop to map all 512 entries
    ; inside the page table
    inc ecx ; i++
    cmp ecx, 512 ; i != 512
    jne .map_page_table  ; loop!

    ret

enable_paging:
    ; load page map table to cr3 reg
    mov eax, page_map_table
    mov cr3, eax

    ; enable physical address extension flag to cr4 reg
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the MSR
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 reg
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

; Align each table
section .bss
align 4096
page_map_table:
    resb 4096
page_pointer_table:
    resb 4096
page_table:
    resb 4096
stack_bottom:
    resb 64
stack_top:

; Global Descriptor Table Setup
section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
