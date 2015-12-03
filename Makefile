BUILDDIR=build

ASM_MODULES=src/boot.o src/long_mode.o
RUST_LIB=target/debug/libkern.a

LDFLAGS=-n --gc-sections -Tlink.ld -melf_x86_64
RUSTFLAGS=-g

AS=nasm
ASFLAGS=-felf64

.PHONY: all clean link iso

all: $(ASM_MODULES) $(RUST_LIB) link iso

$(RUST_LIB):
	@cargo rustc -- -Z no-landing-pads --verbose -C no-redzone

link:
	ld $(LDFLAGS) -o $(BUILDDIR)/kernel $(ASM_MODULES) $(RUST_LIB)

iso:
	mkdir -p iso/boot/grub
	cp $(BUILDDIR)/kernel iso/boot
	cp grub.cfg iso/boot/grub
	grub-mkrescue -o build/kernel.iso iso

clean:
	rm -f $(ASM_MODULES)
	@cargo clean
