SOURCES=src/boot.o src/common.o src/framebuffer.o src/main.o
CFLAGS=-m32 -ggdb -nostdlib -nostdinc -fno-builtin -fno-stack-protector
LDFLAGS=-Tlink.ld -melf_i386
AS=nasm
ASFLAGS=-felf
BUILDDIR= build

all: $(SOURCES) link

link:
	ld $(LDFLAGS) -o $(BUILDDIR)/kernel $(SOURCES)

clean:
	-rm src/*.o $(BUILDDIR)/*
