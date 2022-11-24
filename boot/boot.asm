global start
extern long_mode_start

section .text
bits 32
start:
    mov esp, stack_top

    call set_up_page_tables
    call enable_paging

    ; load the 64-bit GDT
    lgdt [gdt64.pointer]

    jmp gdt64.code:long_mode_start

    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

set_up_page_tables:
    ; map first page-map table to page pointer table
    mov eax, page_pointer_table
    or eax, 0b11 ; present + writable
    mov [page_map_table], eax

    ; map first page pointer table entry to page table table
    mov eax, page_table
    or eax, 0b11 ; present + writable
    mov [page_pointer_table], eax

    ; map each page table entry to a huge 2MiB page
    mov ecx, 0         ; counter variable

.map_page_table:
    ; map ecx-th page_table entry to a huge page that starts at address 2MiB*ecx
    mov eax, 0x200000  ; 2MiB
    mul ecx            ; start address of ecx-th page
    or eax, 0b10000011 ; present + writable + huge
    mov [page_table + ecx * 8], eax ; map ecx-th entry

    inc ecx            ; increase counter
    cmp ecx, 512       ; if counter == 512, the whole page table table is mapped
    jne .map_page_table  ; else map the next entry

    ret

enable_paging:
    ; load page map table to cr3 register (cpu uses this to access the page map table)
    mov eax, page_map_table
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

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

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; new
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
