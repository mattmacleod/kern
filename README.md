# Kernel experiment

## Useful references

http://www.gnu.org/software/grub/manual/multiboot/html_node/multiboot.h.html
http://www.jamesmolloy.co.uk
http://os.phil-opp.com
https://github.com/thepowersgang/rust-barebones-kernel

## How to

```
vagrant up
vagrant ssh
sudo apt-get install nasm build-essential curl git xorriso
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
multirust default nightly
make
```
