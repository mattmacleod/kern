SOURCES=src/boot.o src/main.o
CFLAGS=-m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector
LDFLAGS=-Tlink.ld -melf_i386
AS=nasm
ASFLAGS=-felf
BUILDDIR= build

all: $(SOURCES) 
	$(CC) $(CFLAGS) $(LDFLAGS) $(SOURCES) -o $(BUILDDIR)/kernel

clean:
	-rm src/*.o $(BUILDDIR)/kernel
