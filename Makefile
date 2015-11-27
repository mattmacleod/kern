BUILDDIR=build

ASM_MODULES=src/boot.o
RUST_LIB=target/debug/libkern.a

LDFLAGS=--gc-sections -Tlink.ld -melf_i386

AS=nasm
ASFLAGS=-felf

all: $(ASM_MODULES) $(RUST_LIB) link

$(RUST_LIB):
	@cargo rustc -- -Z no-landing-pads --verbose -C no-redzone

link:
	ld $(LDFLAGS) -o $(BUILDDIR)/kernel $(ASM_MODULES) $(RUST_LIB)

clean:
	@cargo clean
