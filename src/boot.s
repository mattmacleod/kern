MBOOT_PAGE_ALIGN    equ 1<<0       ; Load kernel and modules on a page boundary
MBOOT_MEM_INFO      equ 1<<1       ; Provide your kernel with memory info
MBOOT_HEADER_MAGIC  equ 0x1BADB002 ; Multiboot Magic value
MBOOT_HEADER_FLAGS  equ MBOOT_PAGE_ALIGN | MBOOT_MEM_INFO
MBOOT_CHECKSUM      equ -(MBOOT_HEADER_MAGIC + MBOOT_HEADER_FLAGS)

bits 32

section .multiboot_header
mboot:
  dd  MBOOT_HEADER_MAGIC        ; GRUB will search for this value on each
                                ; 4-byte boundary in your kernel file
  dd  MBOOT_HEADER_FLAGS        ; How GRUB should load your file / settings
  dd  MBOOT_CHECKSUM            ; To ensure that the above values are correct
   
  dw 0
  dw 0
  dd 8

global start

section .text
start:
    extern main
    call main
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
