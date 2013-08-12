#!/bin/bash

sudo losetup /dev/loop0 `dirname $0`/../build/floppy.img
sudo mount /dev/loop0 /mnt
sudo cp `dirname $0`/../build/kernel /mnt/kernel
sudo umount /dev/loop0
sudo losetup -d /dev/loop0
