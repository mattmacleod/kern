#!/bin/bash

sudo /sbin/losetup /dev/loop0 `dirname $0`/../build/floppy.img
sudo bochs -f `dirname $0`/bochsrc
sudo /sbin/losetup -d /dev/loop0
