bits 32
global start
extern long_mode_start

MULTIBOOT_HEADER_MAGIC equ 0xE85250D6
MULTIBOOT_ARCH         equ 0
MULTIBOOT_HEADER_SIZE  equ header_end - header_start
MULTIBOOT_CHECKSUM     equ 0x100000000 - (MULTIBOOT_HEADER_MAGIC + MULTIBOOT_ARCH + MULTIBOOT_HEADER_SIZE)

ERR_NO_MULTIBOOT equ 0x30 ; Error 0
ERR_NO_CPUID     equ 0x31 ; Error 1
ERR_NO_LONG_MODE equ 0x32 ; Error 2

MULTIBOOT_MAGIC_TEST equ 0x36d76289

; This is the multiboot header (static)
section .multiboot_header
header_start:
  dd  MULTIBOOT_HEADER_MAGIC
  dd  MULTIBOOT_ARCH
  dd  MULTIBOOT_HEADER_SIZE
  dd  MULTIBOOT_CHECKSUM

  ; Additional multiboot tags would go here

  dw 0
  dw 0
  dd 8
header_end:


; Entry point here
section .text
start:
    mov esp, stack_top ; Move the stack pointer to the top of our stack

    call test_multiboot
    call test_cpuid
    call test_long_mode

    call setup_page_tables 
    call enable_paging

    lgdt [gdt64.pointer]
    mov ax, 16
    mov ss, ax
    mov ds, ax
    mov es, ax

    jmp gdt64.code:long_mode_start



; Checks that we were actually loaded by a multiboot-compliant bootloader, as
; we are about to use some multiboot-specific options
test_multiboot:
    cmp eax, MULTIBOOT_MAGIC_TEST
    jne .test_multiboot_failed
    ret
.test_multiboot_failed:
    mov al, ERR_NO_MULTIBOOT
    jmp error


; This is a bit of code from the OSDev Wiki – it checks that CPUID is available.
; If it isn't, we die.
test_cpuid:
    pushfd                 ; Store the FLAGS-register.
    pop eax                ; Restore the A-register.
    mov ecx, eax           ; Set the C-register to the A-register.
    xor eax, 1 << 21       ; Flip the ID-bit, which is bit 21.
    push eax               ; Store the A-register.
    popfd                  ; Restore the FLAGS-register.
    pushfd                 ; Store the FLAGS-register.
    pop eax                ; Restore the A-register.
    push ecx               ; Store the C-register.
    popfd                  ; Restore the FLAGS-register.
    xor eax, ecx           ; Do a XOR-operation on the A-register and the C-register.
    jz .test_cpuid_failed  ; The zero flag is set, no CPUID.
    ret                    ; CPUID is available for use.
.test_cpuid_failed:
    mov al, ERR_NO_CPUID
    jmp error


; Also from OSDev – checks that long mode is supported (i.e. we are using a 64-
; bit CPU)
test_long_mode:
    mov eax, 0x80000000       ; Set the A-register to 0x80000000.
    cpuid                     ; CPU identification.
    cmp eax, 0x80000001       ; Compare the A-register with 0x80000001.
    jb .test_long_mode_failed ; It is less, there is no long mode.
    mov eax, 0x80000001       ; Set the A-register to 0x80000001.
    cpuid                     ; CPU identification.
    test edx, 1 << 29         ; Test if the LM-bit, which is bit 29, is set in the D-register.
    jz .test_long_mode_failed ; They aren't, there is no long mode.
    ret
.test_long_mode_failed:
    mov al, ERR_NO_LONG_MODE
    jmp error


; Basic error handler – display the word error and an 8-bit error code
error:
    mov byte  [0xb800f], 0x4f
    mov byte  [0xb800e], al
    mov dword [0xb800a], 0x4f204f20
    mov dword [0xb8008], 0x4f3a4f52
    mov dword [0xb8004], 0x4f4F4f52
    mov dword [0xb8000], 0x4f524f45
    hlt


setup_page_tables:
    mov eax, p3_table
    or eax, 0b11 ; present + writable
    mov [p4_table], eax

    ; map first P3 entry to P2 table
    mov eax, p2_table
    or eax, 0b11 ; present + writable
    mov [p3_table], eax

    mov ecx, 0         ; counter variable


.map_p2_table:
    ; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
    mov eax, 0x200000  ; 2MiB
    mul ecx            ; start address of ecx-th page
    or eax, 0b10000011 ; present + writable + huge
    mov [p2_table + ecx * 8], eax ; map ecx-th entry

    inc ecx            ; increase counter
    cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
    jne .map_p2_table  ; else map the next entry

    ret


enable_paging:
    ; load P4 to cr3 register (cpu uses this to access the P4 table)
    mov eax, p4_table
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


section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    ; Code
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
    ; Data
    dq (1<<44) | (1<<47) | (1<<41)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64


; Create the stack and page tables
section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 64
stack_top:
