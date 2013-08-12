#!/bin/bash

qemu-system-i386 -k en-gb -gdb tcp::1234 -fda /tmp/floppy.img -boot a -no-fd-bootchk 
