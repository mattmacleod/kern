#!/bin/bash

qemu-system-x86_64 -drive format=raw,file=build/kernel.iso -gdb tcp::1234
