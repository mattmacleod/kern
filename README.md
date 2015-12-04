# Kernel experiment

## Useful references

http://nongnu.askapache.com/grub/phcoder/multiboot.pdf
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
