## Kernel experiment

Developed from James Molloy's demo kernel: http://www.jamesmolloy.co.uk

## How to

```
vagrant up
vagrant ssh
sudo apt-get install nasm build-essential curl git
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
multirust default nightly
make
```
