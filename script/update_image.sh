#!/bin/bash

sudo cp `dirname $0`/../test/floppy.img tmp/floppy.img
sudo mount -o loop tmp/floppy.img /mnt
sudo cp `dirname $0`/../build/kernel /mnt/kernel
sudo umount /mnt
