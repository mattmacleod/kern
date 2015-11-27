bits 32
global start

MULTIBOOT_HEADER_MAGIC equ 0xE85250D6
MULTIBOOT_ARCH         equ 0
MULTIBOOT_HEADER_SIZE  equ header_end - header_start
MULTIBOOT_CHECKSUM     equ 0x100000000 - (MULTIBOOT_HEADER_MAGIC + MULTIBOOT_ARCH + MULTIBOOT_HEADER_SIZE)

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

section .text
start:
    extern main
    call main
    hlt
